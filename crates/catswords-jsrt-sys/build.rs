struct CKRBuilder {
    out_dir: String,
}

fn main() {
    let out_dir     : String = std::env::var("OUT_DIR").unwrap();
    let builder     : CKRBuilder = CKRBuilder {
        out_dir : format!("{out_dir}/ckr")
    };

    println!("cargo:rerun-if-env-changed=CHAKRACORE_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=CHAKRACORE_LIB_DIR");

    match std::env::var("CHAKRACORE_INCLUDE_DIR") {
        Ok( _) => {
            println!("CHAKRACORE_INCLUDE_DIR has set. (not very meaningful though)");
        }
        Err(_) =>  { }
    }

    match std::env::var("CHAKRACORE_LIB_DIR") {
        Ok(path) => {
            println!("There is chakracore somewhere {}", path);
            println!("cargo:rustc-link-search=native={}/", path);
            println!("cargo:rustc-link-search=native={}/bin", path);
            println!("cargo:rustc-link-search=native={}/lib", path);
            println!("cargo:rustc-link-lib=dylib=ChakraCore");
            return;
        }
        Err(_) => {}
    }

    builder.fetch()
        .expect("Failed to fetch ChakraCore: https://github.com/chakra-core/ChakraCore")
        .build()
        .expect("");
}

/* TODO: change check_vs, need support cmd and PS. and separate about build.
*/

fn visual_studio_exists() -> bool {
    let check_vs = std::process::Command::new("vswhere")
        .args(["-latest",
            "-requires", "Microsoft.VisualStudio.Workload.NativeDesktop",
            "-property", "installationPath",])
        .output();

    match check_vs {
        Ok(o) =>{ eprintln!("{}", String::from_utf8_lossy(&o.stdout));
            !o.stdout.is_empty()}, 
        Err(_) => false,
    }
}

impl CKRBuilder {
    fn fetch(&self) -> Result<&CKRBuilder, std::io::Error> {
        let r : Result<std::process::Output, std::io::Error> = std::process::Command::new("git")
            .args(["clone", "https://github.com/chakra-core/ChakraCore", self.out_dir.as_str(), "--depth", "1"])
            .output();

        match r {
            Ok( _) => return Ok(self),
            Err(e) => return Err(e)
        }
    }


    #[cfg(target_os = "windows")]
    fn build(&self) -> Result<&CKRBuilder, std::io::Error> {

        match visual_studio_exists() {
            false => return self.build_fallback(),
            true => {
                #[cfg(target_arch = "x86_64")]
                let arch : &str = "x64";
                #[cfg(target_arch = "x86")]
                let arch : &str = "Win32";
                #[cfg(target_arch = "aarch64")]
                let arch : &str = "ARM64";

                let submodule_install = std::process::Command::new("git")
                    .args(["submodule", "update", "--init", "--recursive"])
                    .output();
                match submodule_install {
                    Ok(o) => {
                        println!("{}", String::from_utf8_lossy(&o.stdout));
                        println!("{}", String::from_utf8_lossy(&o.stderr));
                    }
                    Err(_)=> {
                        return self.build_fallback();
                    }
                }

                let result = std::process::Command::new("msbuild")
                    .args(["Build\\Chakra.Core.sln",
                        "/p:Configuration=Release",
                        &format!("/p:Platform={}", arch),
                        "/m",
                    ])
                    .current_dir(self.out_dir.as_str())
                    .output();

                match result {
                    Ok(o) => {
                        eprintln!("Build Success");
                        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                        return Ok(self);
                    }
                    Err(e) => {
                        println!("Failed to build ChakraCore: {}", e);
                        return self.build_fallback();
                    }
                }
            }
        }
    }


    #[cfg(not(target_os = "windows"))]
    fn build(&self) -> Result<&CKRBuilder, std::io::Error> {
        return self.build_fallback();
    }
    fn build_fallback(&self) -> Result<&CKRBuilder, std::io::Error> {
        let dst = cmake::Config::new(self.out_dir.as_str())
            .define("CMAKE_C_COMPILER", "clang")
            .define("CMAKE_CXX_COMPILER", "clang++")
            .define("CMAKE_ASM_COMPILER", "clang")
            .build_target("all")
            .build();

        println!("cargo:rustc-link-search=native={}/build/", dst.display());
        println!("cargo:rustc-link-search=native={}/build/bin/ChakraCore", dst.display());
        println!("cargo:rustc-link-lib=dylib=ChakraCore");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=icuuc");
        println!("cargo:rustc-link-lib=dylib=icui18n");

        return Ok(self);
    }
}
