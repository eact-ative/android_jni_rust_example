JAVA_HOME = /Applications/Android\ Studio.app/Contents/jre/Contents/Home/bin/
run: lib
	cd ./java && $(JAVA_HOME)javac HelloWorld.java && $(JAVA_HOME)java -Djava.library.path=../target/debug/ HelloWorld

.PHONY: lib

javah:
	$(JAVA_HOME)javah ./HelloWorld

lib:
	cargo build