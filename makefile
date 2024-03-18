run:
	rm -rf target/ && cargo pgrx run 
package:
	rm -rf target/ && cargo pgrx package --pg-config $(HOME)/.pgrx/15.6/pgrx-install/bin/pg_config
deploy:
	rm -rf target/ && cargo pgrx package --pg-config /Applications/Postgres.app/Contents/Versions/15/bin/pg_config --out-dir ./packages