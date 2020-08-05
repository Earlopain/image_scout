project_name := "reverser"
target_folder := "target"
template_folder := "templates"
static_folder := "static"

run:
    cargo run

reseed:
    diesel migration redo
    cargo run -p seeding

build: (_sync_resources "debug")
    cargo build

build-release: (_sync_resources "release")
    cargo build --release

package: build ( _zip_build "debug")

package_release: build-release (_zip_build "release")

_sync_resources build_type:
    rsync -avu --delete "{{template_folder}}/" "{{target_folder}}/{{build_type}}/{{template_folder}}"
    rsync -avu --delete "{{static_folder}}/" "{{target_folder}}/{{build_type}}/{{static_folder}}"

@_zip_build build_type:
    cd target/{{build_type}}/ && rm -f {{build_type}}.zip && zip -r {{build_type}}.zip {{project_name}} {{template_folder}}/ {{static_folder}}/
