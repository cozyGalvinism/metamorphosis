#[macro_use]
extern crate custom_error;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod clients;
pub mod models;
mod validators;

pub use clients::mojang::MojangUpdater;

#[cfg(test)]
mod tests {
    use crate::models::{mojang::MojangVersionFile, forge::ForgeVersionFile};

    #[test]
    fn test_version_parse() {
        use serde::{Deserialize, Serialize};

        let json = r#"
        {
            "_comment_": [
              "Please do not automate the download and installation of Forge.",
              "Our efforts are supported by ads from the download page.",
              "If you MUST automate this, please consider supporting the project through https://www.patreon.com/LexManos/"
            ],
            "id": "1.14.3-forge-27.0.47",
            "time": "2019-07-10T02:24:03+00:00",
            "releaseTime": "2019-07-10T02:24:03+00:00",
            "type": "release",
            "mainClass": "cpw.mods.modlauncher.Launcher",
            "inheritsFrom": "1.14.3",
            "logging": {},
            "arguments": {
              "game": [
                "--launchTarget",
                "fmlclient",
                "--fml.forgeVersion",
                "27.0.47",
                "--fml.mcVersion",
                "1.14.3",
                "--fml.forgeGroup",
                "net.minecraftforge",
                "--fml.mcpVersion",
                "20190624.152911"
              ]
            },
            "libraries": [
              {
                "name": "net.minecraftforge:forge:1.14.3-27.0.47",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/forge/1.14.3-27.0.47/forge-1.14.3-27.0.47.jar",
                    "url": "",
                    "sha1": "1914f24b5ba6a5a20cd67c1b9a3ece02bc18fa34",
                    "size": 163781
                  }
                }
              },
              {
                "name": "org.ow2.asm:asm:6.2",
                "downloads": {
                  "artifact": {
                    "path": "org/ow2/asm/asm/6.2/asm-6.2.jar",
                    "url": "https://maven.minecraftforge.net/org/ow2/asm/asm/6.2/asm-6.2.jar",
                    "sha1": "1b6c4ff09ce03f3052429139c2a68e295cae6604",
                    "size": 111214
                  }
                }
              },
              {
                "name": "org.ow2.asm:asm-commons:6.2",
                "downloads": {
                  "artifact": {
                    "path": "org/ow2/asm/asm-commons/6.2/asm-commons-6.2.jar",
                    "url": "https://maven.minecraftforge.net/org/ow2/asm/asm-commons/6.2/asm-commons-6.2.jar",
                    "sha1": "f0df1c69e34a0463679d7c8db36ddb4312836e76",
                    "size": 78919
                  }
                }
              },
              {
                "name": "org.ow2.asm:asm-tree:6.2",
                "downloads": {
                  "artifact": {
                    "path": "org/ow2/asm/asm-tree/6.2/asm-tree-6.2.jar",
                    "url": "https://maven.minecraftforge.net/org/ow2/asm/asm-tree/6.2/asm-tree-6.2.jar",
                    "sha1": "61570e046111559f38d4e0e580c005f75988c0a6",
                    "size": 50370
                  }
                }
              },
              {
                "name": "cpw.mods:modlauncher:2.1.5",
                "downloads": {
                  "artifact": {
                    "path": "cpw/mods/modlauncher/2.1.5/modlauncher-2.1.5.jar",
                    "url": "https://maven.minecraftforge.net/cpw/mods/modlauncher/2.1.5/modlauncher-2.1.5.jar",
                    "sha1": "d8606a45c22002d17a74c65f1d8a950240324f4d",
                    "size": 101896
                  }
                }
              },
              {
                "name": "cpw.mods:grossjava9hacks:1.1.0",
                "downloads": {
                  "artifact": {
                    "path": "cpw/mods/grossjava9hacks/1.1.0/grossjava9hacks-1.1.0.jar",
                    "url": "https://maven.minecraftforge.net/cpw/mods/grossjava9hacks/1.1.0/grossjava9hacks-1.1.0.jar",
                    "sha1": "ba432ff9b3477370a2317eff223ad3b6a82d37b1",
                    "size": 1759
                  }
                }
              },
              {
                "name": "net.minecraftforge:accesstransformers:0.16.0-shadowed",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/accesstransformers/0.16.0-shadowed/accesstransformers-0.16.0-shadowed.jar",
                    "url": "https://maven.minecraftforge.net/net/minecraftforge/accesstransformers/0.16.0/accesstransformers-0.16.0-shadowed.jar",
                    "sha1": "a87a163b87fd3ce583038f56d4e8bb9c97ca5228",
                    "size": 444428
                  }
                }
              },
              {
                "name": "net.minecraftforge:eventbus:0.10.3-milestone.0.1+1a5fa31-service",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/eventbus/0.10.3-milestone.0.1+1a5fa31-service/eventbus-0.10.3-milestone.0.1+1a5fa31-service.jar",
                    "url": "https://maven.minecraftforge.net/net/minecraftforge/eventbus/0.10.3-milestone.0.1+1a5fa31/eventbus-0.10.3-milestone.0.1+1a5fa31-service.jar",
                    "sha1": "568b9d2fc8a1c4435c50bfc5a184ee279b2a31f5",
                    "size": 39599
                  }
                }
              },
              {
                "name": "net.minecraftforge:forgespi:0.13.0",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/forgespi/0.13.0/forgespi-0.13.0.jar",
                    "url": "https://maven.minecraftforge.net/net/minecraftforge/forgespi/0.13.0/forgespi-0.13.0.jar",
                    "sha1": "2623344ba8cdb3e2e0a16be59c1fc22ad25f2981",
                    "size": 16607
                  }
                }
              },
              {
                "name": "net.minecraftforge:coremods:0.6.3-milestone.0.4+c1d1f08",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/coremods/0.6.3-milestone.0.4+c1d1f08/coremods-0.6.3-milestone.0.4+c1d1f08.jar",
                    "url": "https://maven.minecraftforge.net/net/minecraftforge/coremods/0.6.3-milestone.0.4+c1d1f08/coremods-0.6.3-milestone.0.4+c1d1f08.jar",
                    "sha1": "9a59b6225a37c2e03f6f394c5a32a8932fa2d6c4",
                    "size": 18630
                  }
                }
              },
              {
                "name": "net.minecraftforge:unsafe:0.2.0",
                "downloads": {
                  "artifact": {
                    "path": "net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar",
                    "url": "https://maven.minecraftforge.net/net/minecraftforge/unsafe/0.2.0/unsafe-0.2.0.jar",
                    "sha1": "54d7a0a5e8fdb71b973025caa46f341ae5904f39",
                    "size": 2834
                  }
                }
              },
              {
                "name": "com.electronwill.night-config:core:3.6.0",
                "downloads": {
                  "artifact": {
                    "path": "com/electronwill/night-config/core/3.6.0/core-3.6.0.jar",
                    "url": "https://maven.minecraftforge.net/com/electronwill/night-config/core/3.6.0/core-3.6.0.jar",
                    "sha1": "0412f2edf0ef6cc9178ab38751af4e507f413bef",
                    "size": 199763
                  }
                }
              },
              {
                "name": "com.electronwill.night-config:toml:3.6.0",
                "downloads": {
                  "artifact": {
                    "path": "com/electronwill/night-config/toml/3.6.0/toml-3.6.0.jar",
                    "url": "https://maven.minecraftforge.net/com/electronwill/night-config/toml/3.6.0/toml-3.6.0.jar",
                    "sha1": "f0c2c48748bdfc8d1f3798ec478f003bccc7e0b1",
                    "size": 31256
                  }
                }
              },
              {
                "name": "org.jline:jline:3.9.0",
                "downloads": {
                  "artifact": {
                    "path": "org/jline/jline/3.9.0/jline-3.9.0.jar",
                    "url": "https://maven.minecraftforge.net/org/jline/jline/3.9.0/jline-3.9.0.jar",
                    "sha1": "da6eb8ebdd131ec41f7e42e7e77b257868279698",
                    "size": 707273
                  }
                }
              },
              {
                "name": "org.apache.maven:maven-artifact:3.6.0",
                "downloads": {
                  "artifact": {
                    "path": "org/apache/maven/maven-artifact/3.6.0/maven-artifact-3.6.0.jar",
                    "url": "https://maven.minecraftforge.net/org/apache/maven/maven-artifact/3.6.0/maven-artifact-3.6.0.jar",
                    "sha1": "d4c0da647de59c9ccc304a112fe1f1474d49e8eb",
                    "size": 55051
                  }
                }
              },
              {
                "name": "net.jodah:typetools:0.6.0",
                "downloads": {
                  "artifact": {
                    "path": "net/jodah/typetools/0.6.0/typetools-0.6.0.jar",
                    "url": "https://maven.minecraftforge.net/net/jodah/typetools/0.6.0/typetools-0.6.0.jar",
                    "sha1": "a1552ecd6f5b9444585a7c4b05f6312cf7d32fd3",
                    "size": 14734
                  }
                }
              },
              {
                "name": "java3d:vecmath:1.5.2",
                "downloads": {
                  "artifact": {
                    "path": "java3d/vecmath/1.5.2/vecmath-1.5.2.jar",
                    "url": "https://libraries.minecraft.net/java3d/vecmath/1.5.2/vecmath-1.5.2.jar",
                    "sha1": "79846ba34cbd89e2422d74d53752f993dcc2ccaf",
                    "size": 318956
                  }
                }
              },
              {
                "name": "org.apache.logging.log4j:log4j-api:2.11.2",
                "downloads": {
                  "artifact": {
                    "path": "org/apache/logging/log4j/log4j-api/2.11.2/log4j-api-2.11.2.jar",
                    "url": "https://maven.minecraftforge.net/org/apache/logging/log4j/log4j-api/2.11.2/log4j-api-2.11.2.jar",
                    "sha1": "f5e9a2ffca496057d6891a3de65128efc636e26e",
                    "size": 266283
                  }
                }
              },
              {
                "name": "org.apache.logging.log4j:log4j-core:2.11.2",
                "downloads": {
                  "artifact": {
                    "path": "org/apache/logging/log4j/log4j-core/2.11.2/log4j-core-2.11.2.jar",
                    "url": "https://maven.minecraftforge.net/org/apache/logging/log4j/log4j-core/2.11.2/log4j-core-2.11.2.jar",
                    "sha1": "6c2fb3f5b7cd27504726aef1b674b542a0c9cf53",
                    "size": 1629585
                  }
                }
              },
              {
                "name": "net.minecrell:terminalconsoleappender:1.1.1",
                "downloads": {
                  "artifact": {
                    "path": "net/minecrell/terminalconsoleappender/1.1.1/terminalconsoleappender-1.1.1.jar",
                    "url": "https://maven.minecraftforge.net/net/minecrell/terminalconsoleappender/1.1.1/terminalconsoleappender-1.1.1.jar",
                    "sha1": "d7e48a3c5f778bb8b41d178a52b4411dff418a0c",
                    "size": 15240
                  }
                }
              },
              {
                "name": "net.sf.jopt-simple:jopt-simple:5.0.4",
                "downloads": {
                  "artifact": {
                    "path": "net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar",
                    "url": "https://maven.minecraftforge.net/net/sf/jopt-simple/jopt-simple/5.0.4/jopt-simple-5.0.4.jar",
                    "sha1": "4fdac2fbe92dfad86aa6e9301736f6b4342a3f5c",
                    "size": 78146
                  }
                }
              }
            ]
          }
        "#;
        let mojang_version_file = serde_json::from_str::<MojangVersionFile>(json);
        let forge_version_file = serde_json::from_str::<ForgeVersionFile>(json);

        println!("{:?}", mojang_version_file);
        println!("{:?}", forge_version_file);
    }
}
