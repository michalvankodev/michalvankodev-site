port := env_var_or_default('PORT', '3080')

# Tailwind in watch mode
tailwind:
	npx @tailwindcss/cli -i ./styles/input.css -o ./styles/output.css --watch

# Tailwind one-shot build (used in CI)
tailwind_build:
	npx @tailwindcss/cli -i ./styles/input.css -o ./styles/output.css

# svg sprite creation
svgstore:
	npx svgstore -o templates/icons/sprite.svg static/svg/input/*.svg

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

# Warm up the SSG export: crawl all pages once (pages only — image requests are
# skipped) so every image generation job gets enqueued, then wait for the queue
# to drain before the real crawl. Guarantees complete generated images in dist/.
ssg_prewarm:
	#!/usr/bin/env bash
	set -euxo pipefail
	rm -rf dist_warmup
	# 404s are tolerated: generated images may not exist yet — that is the point
	# of the warmup; the queue drains before the real crawl starts.
	wget --no-convert-links -r --reject jpg,jpeg,png,avif,gif,webp,svg -P dist_warmup --no-host-directories 127.0.0.1:{{port}} || true
	curl --fail --silent --show-error --max-time 900 http://127.0.0.1:{{port}}/export-wait
	rm -rf dist_warmup

# SSG
ssg: ssg_prewarm
	- wget --no-convert-links -r -p -E -P dist --no-host-directories 127.0.0.1:{{port}}
	- wget --no-convert-links --content-on-error -p -E -P dist --no-host-directories 127.0.0.1:{{port}}/not-found
	- wget --no-convert-links -p -E -P dist --no-host-directories 127.0.0.1:{{port}}/showcase/m-logo-svg
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
