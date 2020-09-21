

#[test]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn compare_with_cupid() {
    let information = cupid::master().unwrap();
    assert_eq!(core_detect::is_x86_feature_detected!("aes"), information.aesni());
    assert_eq!(
        core_detect::is_x86_feature_detected!("pclmulqdq"),
        information.pclmulqdq()
    );
    assert_eq!(core_detect::is_x86_feature_detected!("rdrand"), information.rdrand());
    assert_eq!(core_detect::is_x86_feature_detected!("rdseed"), information.rdseed());
    assert_eq!(core_detect::is_x86_feature_detected!("tsc"), information.tsc());
    assert_eq!(core_detect::is_x86_feature_detected!("sse"), information.sse());
    assert_eq!(core_detect::is_x86_feature_detected!("sse2"), information.sse2());
    assert_eq!(core_detect::is_x86_feature_detected!("sse3"), information.sse3());
    assert_eq!(core_detect::is_x86_feature_detected!("ssse3"), information.ssse3());
    assert_eq!(core_detect::is_x86_feature_detected!("sse4.1"), information.sse4_1());
    assert_eq!(core_detect::is_x86_feature_detected!("sse4.2"), information.sse4_2());
    assert_eq!(core_detect::is_x86_feature_detected!("sse4a"), information.sse4a());
    assert_eq!(core_detect::is_x86_feature_detected!("sha"), information.sha());
    assert_eq!(core_detect::is_x86_feature_detected!("avx"), information.avx());
    assert_eq!(core_detect::is_x86_feature_detected!("avx2"), information.avx2());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512f"), information.avx512f());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512cd"), information.avx512cd());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512er"), information.avx512er());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512pf"), information.avx512pf());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512bw"), information.avx512bw());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512dq"), information.avx512dq());
    assert_eq!(core_detect::is_x86_feature_detected!("avx512vl"), information.avx512vl());
    assert_eq!(
        core_detect::is_x86_feature_detected!("avx512ifma"),
        information.avx512_ifma()
    );
    assert_eq!(
        core_detect::is_x86_feature_detected!("avx512vbmi"),
        information.avx512_vbmi()
    );
    assert_eq!(
        core_detect::is_x86_feature_detected!("avx512vpopcntdq"),
        information.avx512_vpopcntdq()
    );
    assert_eq!(core_detect::is_x86_feature_detected!("fma"), information.fma());
    assert_eq!(core_detect::is_x86_feature_detected!("bmi1"), information.bmi1());
    assert_eq!(core_detect::is_x86_feature_detected!("bmi2"), information.bmi2());
    assert_eq!(core_detect::is_x86_feature_detected!("popcnt"), information.popcnt());
    assert_eq!(core_detect::is_x86_feature_detected!("abm"), information.lzcnt());
    assert_eq!(core_detect::is_x86_feature_detected!("tbm"), information.tbm());
    assert_eq!(core_detect::is_x86_feature_detected!("lzcnt"), information.lzcnt());
    assert_eq!(core_detect::is_x86_feature_detected!("xsave"), information.xsave());
    assert_eq!(core_detect::is_x86_feature_detected!("xsaveopt"), information.xsaveopt());
    assert_eq!(
        core_detect::is_x86_feature_detected!("xsavec"),
        information.xsavec_and_xrstor()
    );
    assert_eq!(
        core_detect::is_x86_feature_detected!("xsaves"),
        information.xsaves_xrstors_and_ia32_xss()
    );
    assert_eq!(
        core_detect::is_x86_feature_detected!("cmpxchg16b"),
        information.cmpxchg16b(),
    );
    assert_eq!(core_detect::is_x86_feature_detected!("adx"), information.adx(),);
    assert_eq!(core_detect::is_x86_feature_detected!("rtm"), information.rtm(),);
}
