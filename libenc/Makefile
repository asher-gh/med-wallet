java_run: lib
	javac com/medwalletapp/RustBridgeModule.java && java -cp . -Djava.library.path=../target/release/ com.medwalletapp.RustBridgeModule

.PHONY: lib

javah:
	javac -h . com/medwalletapp/RustBridgeModule.java

lib:
	cargo build -r -p fhe_enc --quiet
