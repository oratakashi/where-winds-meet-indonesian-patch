use super::*;

fn sample_shard() -> Vec<u8> {
    build_shard(
        16,
        0x1234,
        &[
            (0, 0xaaaa_0001, b"The leaf?"),
            (3, 0xbbbb_0002, b"Hello"),
            (7, 0xcccc_0003, b"Hello"),
            (15, 0xdddd_0004, TOMBSTONE),
        ],
    )
}

#[test]
fn slots_offset_is_8_aligned() {
    for cap in [1usize, 3, 7, 15, 16, 31, 1023] {
        let o = slots_off(cap);
        assert_eq!(o % 8, 0);
        assert!(o >= 24 + cap + 16 && o < 24 + cap + 16 + 8);
    }
}

#[test]
fn shard_round_trip() {
    let blk = sample_shard();
    let (cap, seed, ents) = parse_shard(&blk).unwrap();
    assert_eq!((cap, seed), (16, 0x1234));
    let got: Vec<_> = ents
        .iter()
        .map(|e| (e.slot, e.hash, e.value.to_vec()))
        .collect();
    assert_eq!(
        got,
        vec![
            (0, 0xaaaa_0001, b"The leaf?".to_vec()),
            (3, 0xbbbb_0002, b"Hello".to_vec()),
            (7, 0xcccc_0003, b"Hello".to_vec()),
            (15, 0xdddd_0004, TOMBSTONE.to_vec()),
        ]
    );
    // rebuild with the same values is byte-identical
    let vals: std::collections::HashMap<usize, Vec<u8>> =
        ents.iter().map(|e| (e.slot, e.value.to_vec())).collect();
    let again = rebuild_shard(&blk, |i| vals.get(&i).map(|v| v.as_slice())).unwrap();
    assert_eq!(again, blk);
}

#[test]
fn rebuild_keeps_ctrl_hash_and_slots_and_dedups() {
    let blk = sample_shard();
    let new: std::collections::HashMap<usize, &[u8]> = [
        (0usize, &b"Daunnya?"[..]),
        (3, b"Halo"),
        (7, b"Halo"),
        (15, TOMBSTONE),
    ]
    .into_iter()
    .collect();
    let out = rebuild_shard(&blk, |i| new.get(&i).copied()).unwrap();
    let s = slots_off(16);
    // header, ctrl and keyHashes untouched
    assert_eq!(out[..24 + 16 + 16], blk[..24 + 16 + 16]);
    for i in 0..16 {
        assert_eq!(
            out[s + i * 16..s + i * 16 + 8],
            blk[s + i * 16..s + i * 16 + 8]
        );
    }
    let (_, _, ents) = parse_shard(&out).unwrap();
    let vals: Vec<&[u8]> = ents.iter().map(|e| e.value).collect();
    assert_eq!(vals, vec![&b"Daunnya?"[..], b"Halo", b"Halo", TOMBSTONE]);
    // "Halo" stored once: blob = "Daunnya?" + "Halo" + 0xFF
    assert_eq!(out.len(), s + 16 * 16 + 8 + 4 + 1);
}

#[test]
fn rel_offset_is_relative_to_the_field() {
    // Regression for the classic mistake: treating relOffset as absolute garbles text.
    let blk = sample_shard();
    let s = slots_off(16);
    let p = s + 3 * 16;
    let rel = u32::from_le_bytes(blk[p + 8..p + 12].try_into().unwrap()) as usize;
    let ln = u32::from_le_bytes(blk[p + 12..p + 16].try_into().unwrap()) as usize;
    assert_eq!(&blk[p + 8 + rel..p + 8 + rel + ln], b"Hello");
    assert_ne!(blk.get(rel..rel + ln), Some(&b"Hello"[..]));
}

#[test]
fn shard_errors() {
    let mut blk = sample_shard();
    blk[24 + 16] = 0; // sentinel
    assert_eq!(
        parse_shard(&blk).unwrap_err().0,
        "sentinel hilang — layout tidak cocok"
    );
    let mut blk = sample_shard();
    blk[8] = 9; // size
    assert_eq!(
        parse_shard(&blk).unwrap_err().0,
        "jumlah slot terisi != size"
    );
    let blk = sample_shard();
    assert!(rebuild_shard(&blk, |_| None).is_err());
}

fn container(hdr: &Header) -> (Vec<Vec<u8>>, Vec<u8>) {
    let blocks = vec![
        build_index(4, 2),
        sample_shard(),
        build_shard(4, 7, &[(1, 0x42, b"x")]),
    ];
    let bytes = write_container_bytes(hdr, &blocks, 3).unwrap();
    (blocks, bytes)
}

#[test]
fn container_v1_round_trip() {
    let (blocks, bytes) = container(&Header::v1());
    let (hdr, got) = read_container_bytes(&bytes, &Keystream::Bytes(&[])).unwrap();
    assert_eq!(hdr, Header::v1());
    assert_eq!(got, blocks);
    assert_eq!(parse_index(&got[0]).unwrap(), (4, 2));
    // layout: 16-byte header + 3 offsets, last offset = payload length
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 3);
    let last = u32::from_le_bytes(bytes[24..28].try_into().unwrap()) as usize;
    assert_eq!(16 + 12 + last, bytes.len());
}

#[test]
fn container_v2_mode4_round_trip() {
    let hdr = Header {
        version: 2,
        mode: MODE_ZSTD,
        enc_dict: None,
        zdict: None,
    };
    let (blocks, bytes) = container(&hdr);
    let (h2, got) = read_container_bytes(&bytes, &Keystream::Bytes(&[])).unwrap();
    assert_eq!(h2, hdr);
    assert_eq!(got, blocks);
}

fn trained_dict() -> Vec<u8> {
    let samples: Vec<Vec<u8>> = (0..400)
        .map(|i| format!("{{\"text\": \"sample string number {i} with shared words Hexi Kaifeng Qinghe {}\"}}", i * 7).into_bytes())
        .collect();
    zstd::dict::from_samples(&samples, 4096).unwrap()
}

#[test]
fn container_v2_mode6_with_encrypted_dictionary() {
    let plain = trained_dict();
    assert_eq!(plain[..4], ZDICT_MAGIC);
    let ks: Vec<u8> = (0..plain.len() + 50).map(|i| (i * 31 + 7) as u8).collect();
    let enc = xor(&plain, &ks[..plain.len()]);
    let hdr = Header {
        version: 2,
        mode: MODE_ZSTD_DICT,
        enc_dict: Some(enc.clone()),
        zdict: Some(plain.clone()),
    };
    let (blocks, bytes) = container(&hdr);
    // encrypted dictionary stored verbatim, blocks use codec 5
    assert_eq!(&bytes[16..16 + enc.len()], &enc[..]);
    let (h2, got) = read_container_bytes(&bytes, &Keystream::Bytes(&ks)).unwrap();
    assert_eq!(h2, hdr);
    assert_eq!(got, blocks);
    let first_block = 16 + enc.len() + 8 + 3 * 4;
    assert_eq!(bytes[first_block], CODEC_ZSTD_DICT);
    // wrong keystream -> invalid dictionary
    let bad = vec![0u8; ks.len()];
    assert_eq!(
        read_container_bytes(&bytes, &Keystream::Bytes(&bad))
            .unwrap_err()
            .0,
        "kamus hasil dekripsi tidak valid (keystream salah?)"
    );
    // short keystream
    let e = read_container_bytes(&bytes, &Keystream::Bytes(&ks[..10]))
        .unwrap_err()
        .0;
    assert!(e.starts_with("keystream 10 byte < kamus"), "{e}");
    // missing keystream file
    let e = read_container_bytes(&bytes, &Keystream::Path("does/not/exist.bin".into()))
        .unwrap_err()
        .0;
    assert!(e.contains("butuh keystream kamus"), "{e}");
}

#[test]
fn container_errors() {
    let (_, bytes) = container(&Header::v1());
    let mut b = bytes.clone();
    b[0] = 0;
    assert!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0
            .starts_with("magic tidak cocok")
    );
    let mut b = bytes.clone();
    b[4] = 9;
    assert_eq!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0,
        "versi container tak dikenal: 9"
    );
    let mut b = bytes.clone();
    b.push(0);
    assert_eq!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0,
        "ukuran file tidak konsisten dengan tabel offset"
    );
    let mut b = bytes.clone();
    b[8] = 0;
    assert_eq!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0,
        "container tanpa blok (blockCount = 0)"
    );
    let mut b = bytes.clone();
    b[16 + 12] = 3; // codec of block 0
    assert_eq!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0,
        "codec tak dikenal: 3"
    );
    let mut b = bytes;
    b[16 + 12] = CODEC_ZSTD_DICT; // dict codec in a file without dictionary
    assert_eq!(
        read_container_bytes(&b, &Keystream::Bytes(&[]))
            .unwrap_err()
            .0,
        "codec tak dikenal: 5"
    );
}

fn read_err(b: &[u8]) -> String {
    read_container_bytes(b, &Keystream::Bytes(&[]))
        .unwrap_err()
        .0
}

/// Sizes read from the file must be checked before they are used to allocate (#34).
#[test]
fn container_bounds() {
    // v2 PoC from the issue: blockCount = u64::MAX right after the header
    let mut poc = Vec::new();
    for v in [MAGIC, 2, MODE_ZSTD] {
        poc.extend_from_slice(&v.to_le_bytes());
    }
    poc.extend_from_slice(&u64::MAX.to_le_bytes());
    assert!(read_err(&poc).starts_with("tabel offset (18446744073709551615 blok) melebihi"));

    // v1 with a huge u32 blockCount
    let (_, bytes) = container(&Header::v1());
    let mut b = bytes.clone();
    b[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(read_err(&b).starts_with("tabel offset (4294967295 blok) melebihi"));

    // block 0 declares a decompressed size over the per-block limit
    let blk0 = 16 + 12; // v1 header + 3 offsets
    let mut b = bytes.clone();
    b[blk0 + 5..blk0 + 9].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(
        read_err(&b),
        format!(
            "blok 0: ukuran hasil dekompresi {} melebihi batas {MAX_BLOCK_RAW}",
            u32::MAX
        )
    );

    // compressed size larger than the block itself
    let mut b = bytes.clone();
    b[blk0 + 1..blk0 + 5].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(read_err(&b), "blok 0: payload terpotong");

    // frame header declares a content size other than the block header's
    let mut b = bytes.clone();
    let us = u32::from_le_bytes(b[blk0 + 5..blk0 + 9].try_into().unwrap());
    b[blk0 + 5..blk0 + 9].copy_from_slice(&(us - 1).to_le_bytes());
    assert_eq!(
        read_err(&b),
        format!("blok 0: ukuran hasil dekompresi {us} != {}", us - 1)
    );

    // offsets going backwards
    let mut b = bytes.clone();
    b[20..24].copy_from_slice(&1u32.to_le_bytes()); // offset of block 1 < block 0 end
    assert_eq!(read_err(&b), "blok 1: offset tidak monoton");

    // block too short for its 9-byte header
    let mut b = bytes;
    b[16..20].copy_from_slice(&4u32.to_le_bytes());
    assert_eq!(read_err(&b), "blok 0: header blok terpotong");

    // total over the container limit: 17 blocks each declaring MAX_BLOCK_RAW
    let n = (MAX_TOTAL_RAW / MAX_BLOCK_RAW) + 1;
    let mut big = Vec::new();
    for v in [MAGIC, 1, n as u32, 0] {
        big.extend_from_slice(&v.to_le_bytes());
    }
    for i in 1..=n {
        big.extend_from_slice(&((i * 9) as u32).to_le_bytes());
    }
    for _ in 0..n {
        big.push(CODEC_ZSTD);
        big.extend_from_slice(&0u32.to_le_bytes());
        big.extend_from_slice(&(MAX_BLOCK_RAW as u32).to_le_bytes());
    }
    assert_eq!(
        read_err(&big),
        format!("total hasil dekompresi melebihi batas {MAX_TOTAL_RAW} byte")
    );
}

#[test]
fn shard_bounds() {
    // size far beyond capacity must be an error, not a huge allocation
    let mut blk = sample_shard();
    blk[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
    assert_eq!(
        parse_shard(&blk).unwrap_err().0,
        "jumlah slot terisi != size"
    );
    // capacity far beyond the block
    let mut blk = sample_shard();
    blk[0..8].copy_from_slice(&u64::MAX.to_le_bytes());
    assert_eq!(parse_shard(&blk).unwrap_err().0, "shard terpotong (ctrl)");
    assert_eq!(
        rebuild_shard(&blk, |_| None).unwrap_err().0,
        "shard terpotong"
    );
}

#[test]
fn file_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("translate_words_map_en");
    let blocks = vec![
        build_index(1, 1),
        build_shard(8, 1, &[(2, 5, "Ünïcødé ✓".as_bytes())]),
    ];
    write_container_ex(&p, &Header::v1(), &blocks, 19).unwrap();
    let (_, got) = read_container_ex(&p, dir.path()).unwrap();
    assert_eq!(got, blocks);
}
