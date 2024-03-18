run:
	cargo pgrx run pg15 --pgcli  
package:
	rm -rf ./target && cargo pgrx package pg15
