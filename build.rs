extern crate embed_resource;
extern crate windows_exe_info;

fn main() {
    embed_resource::compile("SwapMouseButton.rc",embed_resource::NONE).manifest_optional().unwrap();
    windows_exe_info::icon::icon_ico("mouse.ico");
    use windows_exe_info::versioninfo::*;
    // Change these attributes as you need
    VersionInfo {
        file_version: Version(1, 1, 0, 0),
        product_version: Version(1, 1, 0, 0),
        file_flag_mask: FileFlagMask::Win16,
        file_flags: FileFlags {
            debug: false,
            patched: false,
            prerelease: false,
            privatebuild: false,
            infoinferred: false,
            specialbuild: false,
        },
        file_os: FileOS::Windows32,
        file_type: FileType::App,
        file_info: vec![FileInfo {
            lang: Language::SimplifiedChinese,
            charset: CharacterSet::Multilingual,
            comment: None,
            company_name: "sndnvaps.com".into(),
            file_description: String::from("Swap Mouse Button").into(),
            file_version: "1.1.0.0".into(),
            internal_name: "SwapMouseButton".into(),
            legal_copyright: None,
            legal_trademarks: None,
            original_filename: "SwapMouseButton.exe".into(),
            product_name: "SwapMouseButton".into(),
            product_version: "1.1.0.0".into(),
            private_build: None,
            special_build: None,
        }],
    }
    .link()
    .unwrap();
}
