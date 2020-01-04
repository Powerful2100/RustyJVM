static BUILTINS: phf::Map<&'static str, TargetInfo> = ::phf::Map {
    key: 9603444721912725599,
    disps: ::phf::Slice::Static(&[
        (1, 61),
        (1, 1),
        (0, 0),
        (0, 68),
        (0, 50),
        (5, 4),
        (4, 20),
        (4, 95),
        (0, 1),
        (1, 32),
        (1, 6),
        (22, 64),
        (6, 26),
        (2, 97),
        (37, 107),
        (0, 14),
        (2, 0),
        (0, 62),
        (0, 1),
        (0, 2),
        (0, 110),
        (0, 32),
        (16, 80),
    ]),
    entries: ::phf::Slice::Static(&[
        ("i686-unknown-cloudabi", TargetInfo { arch: B("x86"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("powerpc64le-unknown-linux-musl", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("sparc-unknown-linux-gnu", TargetInfo { arch: B("sparc"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i586-unknown-linux-musl", TargetInfo { arch: B("x86"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("mips-unknown-linux-musl", TargetInfo { arch: B("mips"), os: B("linux"), env: B("musl"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("powerpc-unknown-netbsd", TargetInfo { arch: B("powerpc"), os: B("netbsd"), env: B(""), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("riscv32imc-unknown-none-elf", TargetInfo { arch: B("riscv32"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-redox", TargetInfo { arch: B("x86_64"), os: B("redox"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv7a-pc-windows-msvc", TargetInfo { arch: B("arm"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_vendor"), B("pc")), ]) }),
        ("i686-unknown-openbsd", TargetInfo { arch: B("x86"), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7r-none-eabihf", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("thumbv8m.base-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("powerpc64-unknown-freebsd", TargetInfo { arch: B("powerpc64"), os: B("freebsd"), env: B(""), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("wasm32-unknown-emscripten", TargetInfo { arch: B("wasm32"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("mipsisa32r6-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-unknown-hermit", TargetInfo { arch: B("aarch64"), os: B("hermit"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-cloudabi", TargetInfo { arch: B("x86_64"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-unknown-cloudabi", TargetInfo { arch: B("aarch64"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-pc-windows-msvc", TargetInfo { arch: B("x86_64"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("pc")), ]) }),
        ("aarch64-unknown-linux-musl", TargetInfo { arch: B("aarch64"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7-unknown-cloudabi-eabihf", TargetInfo { arch: B("arm"), os: B("cloudabi"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("armv7r-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("powerpc-unknown-linux-musl", TargetInfo { arch: B("powerpc"), os: B("linux"), env: B("musl"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv7neon-linux-androideabi", TargetInfo { arch: B("arm"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-rumprun-netbsd", TargetInfo { arch: B("x86_64"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("rumprun")), ]) }),
        ("armv6-unknown-freebsd", TargetInfo { arch: B("arm"), os: B("freebsd"), env: B("gnueabihf"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("mipsisa64r6-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("riscv64imac-unknown-none-elf", TargetInfo { arch: B("riscv64"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("i686-unknown-haiku", TargetInfo { arch: B("x86"), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-freebsd", TargetInfo { arch: B("x86_64"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-netbsd", TargetInfo { arch: B("x86_64"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv5te-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-dragonfly", TargetInfo { arch: B("x86_64"), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-apple-darwin", TargetInfo { arch: B("x86"), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_feature"), B("sse3")), (B("target_vendor"), B("apple")), ]) }),
        ("x86_64-unknown-uefi", TargetInfo { arch: B("x86_64"), os: B("uefi"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("fxsr")), (B("target_vendor"), B("unknown")), ]) }),
        ("arm-linux-androideabi", TargetInfo { arch: B("arm"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("armebv7r-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("big"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("aarch64-unknown-freebsd", TargetInfo { arch: B("aarch64"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-linux-android", TargetInfo { arch: B("aarch64"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7-linux-androideabi", TargetInfo { arch: B("arm"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-sun-solaris", TargetInfo { arch: B("x86_64"), os: B("solaris"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("sun")), ]) }),
        ("nvptx64-nvidia-cuda", TargetInfo { arch: B("nvptx64"), os: B("cuda"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("nvidia")), ]) }),
        ("mipsel-unknown-linux-musl", TargetInfo { arch: B("mips"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-openbsd", TargetInfo { arch: B("x86_64"), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("wasm32-unknown-unknown", TargetInfo { arch: B("wasm32"), os: B("unknown"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-pc-solaris", TargetInfo { arch: B("x86_64"), os: B("solaris"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("sun")), ]) }),
        ("armv6-unknown-netbsd-eabihf", TargetInfo { arch: B("arm"), os: B("netbsd"), env: B("eabihf"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("s390x-unknown-linux-gnu", TargetInfo { arch: B("s390x"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("wasm32-experimental-emscripten", TargetInfo { arch: B("wasm32"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-fortanix-unknown-sgx", TargetInfo { arch: B("x86_64"), os: B("unknown"), env: B("sgx"), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("fxsr")), (B("target_feature"), B("rdrand")), (B("target_feature"), B("rdseed")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("fortanix")), ]) }),
        ("powerpc64le-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7-unknown-netbsd-eabihf", TargetInfo { arch: B("arm"), os: B("netbsd"), env: B("eabihf"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("mips-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("asmjs-unknown-emscripten", TargetInfo { arch: B("asmjs"), os: B("emscripten"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv7neon-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-unknown-freebsd", TargetInfo { arch: B("x86"), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-linux-musl", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("mipsisa64r6el-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-fuchsia", TargetInfo { arch: B("x86_64"), os: B("fuchsia"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("")), ]) }),
        ("mipsisa32r6el-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-unknown-linux-gnu", TargetInfo { arch: B("x86"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("arm-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("sparc64-unknown-netbsd", TargetInfo { arch: B("sparc64"), os: B("netbsd"), env: B(""), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("mips-unknown-linux-uclibc", TargetInfo { arch: B("mips"), os: B("linux"), env: B("uclibc"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-pc-windows-msvc", TargetInfo { arch: B("x86"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("pc")), ]) }),
        ("i686-unknown-linux-musl", TargetInfo { arch: B("x86"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("i586-pc-windows-msvc", TargetInfo { arch: B("x86"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_vendor"), B("pc")), ]) }),
        ("thumbv6m-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("aarch64-unknown-netbsd", TargetInfo { arch: B("aarch64"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i586-unknown-linux-gnu", TargetInfo { arch: B("x86"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7-unknown-freebsd", TargetInfo { arch: B("arm"), os: B("freebsd"), env: B("gnueabihf"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("msp430-none-elf", TargetInfo { arch: B("msp430"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("16"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("x86_64-linux-android", TargetInfo { arch: B("x86_64"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("popcnt")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_feature"), B("sse3")), (B("target_feature"), B("sse4.1")), (B("target_feature"), B("sse4.2")), (B("target_feature"), B("ssse3")), (B("target_vendor"), B("unknown")), ]) }),
        ("sparcv9-sun-solaris", TargetInfo { arch: B("sparc64"), os: B("solaris"), env: B(""), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("sun")), ]) }),
        ("riscv32imac-unknown-none-elf", TargetInfo { arch: B("riscv32"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("i686-pc-windows-gnu", TargetInfo { arch: B("x86"), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("pc")), ]) }),
        ("aarch64-pc-windows-msvc", TargetInfo { arch: B("aarch64"), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_vendor"), B("pc")), ]) }),
        ("armebv7r-none-eabihf", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("big"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("armv5te-unknown-linux-musleabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-apple-darwin", TargetInfo { arch: B("x86_64"), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_feature"), B("sse3")), (B("target_feature"), B("ssse3")), (B("target_vendor"), B("apple")), ]) }),
        ("mipsel-unknown-linux-gnu", TargetInfo { arch: B("mips"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv8m.main-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("x86_64-unknown-l4re-uclibc", TargetInfo { arch: B("x86_64"), os: B("l4re"), env: B("uclibc"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("arm-unknown-linux-musleabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-unknown-dragonfly", TargetInfo { arch: B("x86"), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("powerpc-unknown-linux-gnuspe", TargetInfo { arch: B("powerpc"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv7m-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("mips64-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("sparc64-unknown-linux-gnu", TargetInfo { arch: B("sparc64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("riscv64gc-unknown-none-elf", TargetInfo { arch: B("riscv64"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("unknown")), ]) }),
        ("mipsel-unknown-linux-uclibc", TargetInfo { arch: B("mips"), os: B("linux"), env: B("uclibc"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-linux-gnux32", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-fuchsia", TargetInfo { arch: B("aarch64"), os: B("fuchsia"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("")), ]) }),
        ("aarch64-unknown-none", TargetInfo { arch: B("aarch64"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("thumbv8m.main-none-eabihf", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("powerpc64-unknown-linux-musl", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("musl"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv4t-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("arm-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-unknown-linux-gnu", TargetInfo { arch: B("aarch64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("thumbv7em-none-eabi", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
        ("powerpc-unknown-linux-gnu", TargetInfo { arch: B("powerpc"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("armv7-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-linux-android", TargetInfo { arch: B("x86"), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_feature"), B("sse3")), (B("target_feature"), B("ssse3")), (B("target_vendor"), B("unknown")), ]) }),
        ("i686-unknown-netbsd", TargetInfo { arch: B("x86"), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("wasm32-wasi", TargetInfo { arch: B("wasm32"), os: B("wasi"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_feature"), B("crt-static")), (B("target_vendor"), B("")), ]) }),
        ("armv7-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("mips64el-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-haiku", TargetInfo { arch: B("x86_64"), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("powerpc64-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-linux-gnu", TargetInfo { arch: B("x86_64"), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-unknown-hermit", TargetInfo { arch: B("x86_64"), os: B("hermit"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("unknown")), ]) }),
        ("aarch64-unknown-openbsd", TargetInfo { arch: B("aarch64"), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("64"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_vendor"), B("unknown")), ]) }),
        ("arm-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32"), switches: B(&[B("unix"), ]), other_keys: B(&[(B("target_family"), B("unix")), (B("target_feature"), B("crt-static")), (B("target_vendor"), B("unknown")), ]) }),
        ("x86_64-pc-windows-gnu", TargetInfo { arch: B("x86_64"), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("64"), switches: B(&[B("windows"), ]), other_keys: B(&[(B("target_family"), B("windows")), (B("target_feature"), B("fxsr")), (B("target_feature"), B("sse")), (B("target_feature"), B("sse2")), (B("target_vendor"), B("pc")), ]) }),
        ("thumbv7em-none-eabihf", TargetInfo { arch: B("arm"), os: B("none"), env: B(""), endian: B("little"), pointer_width: B("32"), switches: B(&[]), other_keys: B(&[(B("target_vendor"), B("")), ]) }),
    ]),
};