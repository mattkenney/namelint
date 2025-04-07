#!/usr/bin/env bash
#
# script to run on localhost
#

set -o errexit
set -o pipefail
set -o nounset

USE_DOCKER=${USE_DOCKER:-false}

echo "INFO: checking for jekyll"
if ! command -v jekyll &> /dev/null; then
	echo "INFO: jekyll not found, using docker"
	USE_DOCKER=true
fi

if [ "$USE_DOCKER" = true ]; then
	echo "INFO: running jekyll in docker"
	docker run --rm -it \
		-v "$(pwd):/srv/jekyll" \
		-p 4000:4000 \
		-e JEKYLL_ENV=development \
		jekyll/builder \
		sh -c "bundle install && bundle exec jekyll serve --drafts --source docs --trace --watch --host=0.0.0.0"

else
	echo "INFO: running jekyll locally"
	jekyll serve \
		--drafts \
		--source docs \
		--trace \
		--watch
fi


#		sh -c "bundle init && bundle add jekyll -v 3.10 && bundle add webrick kramdown-parser-gfm jekyll-redirect-from && bundle exec jekyll serve --drafts --source docs --trace --watch --host=0.0.0.0"
