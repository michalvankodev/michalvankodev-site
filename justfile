port := env_var_or_default('PORT', '3080')

# Tailwind in watch mode
tailwind:
	npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch

# svg sprite creation
svgstore:
   npx svgstore -o templates/icons/sprite.svg src/svg/**.svg

server_dev: 
	cargo watch -x run

# CMS server for local dev
decap_server:
	npx decap-server

# Run dev server in watch mode
dev: 
	#!/usr/bin/env -S parallel --shebang --ungroup --jobs {{ num_cpus() }}
	just server_dev
	just tailwind
	just decap_server

# Run tests
test: 
	cargo test

test_watch:
	cargo watch -x test

# Run server in production mode
prod $TARGET="PROD" $RUST_LOG="info":
    cargo run --release

# Wait for port to listen to connections
wait_for_port:
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! nc -z localhost {{port}}; do
	sleep 3
	done

# Kill the application running on port
kill:
	kill $(pidof axum_server)

# Clean the dist folder
clean:
	rm -rf dist

# SSG
ssg:
	- wget --no-convert-links -r -p -E -P dist --no-host-directories 127.0.0.1:{{port}}
	find generated_images/ -name "*_og*" -exec cp --parents {} dist/ \;
  
# Preview server
preview:
  caddy run --config Caddyfile-preview

# SSG export of production server
export: clean
	just prod &
	just wait_for_port
	just ssg
	just kill

deploy:
 rsync -avz -e ssh ./dist/ michalvanko@katelyn:.config/containers/systemd/michalvankodev-site/dist/ 
