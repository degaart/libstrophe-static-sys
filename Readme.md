# How to use

Add libstrophe-sys-bindgen and libstrophe-static-sys to your dependencies:

```
[dependencies]
libstrophe-sys-bindgen = "5.0.0"
libstrophe-static-sys = { git = "https://github.com/degaart/libstrophe-static-sys.git" }
```

Add the following to your build.rs to handle libstrophe's transitive dependencies:

```
if let Ok(strophe_static_link_libs) = env::var("DEP_STROPHE_STATIC_LINK_LIBS") {
    strophe_static_link_libs.split(",")
        .for_each(|lib| {
            println!("cargo:rustc-link-lib={lib}");
        });
}
```

