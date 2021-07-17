CC	= cargo
PROD_FLAGS	=	--release
PROGRAM	=	img_sync
SERVICE_FILE_DEST	=	/etc/systemd/system/img_sync.service
SERVICE_FILE_SRC	=	./etc/img_sync.service
ETC_FILE_SRC	=	./etc/img_sync_example
ETC_FILE_DST	=	/etc/img_sync
SYSTEMD_INSTALL_DST	=	/usr/local/bin/img_sync
SYSTEMD_INSTALL_SRC	=	./target/release/img_sync


all: $(PROGRAM)

$(PROGRAM): clean test
	$(CC) build $(PROD_FLAGS)

.PHONY: clean
clean:
	$(CC) clean

.PHONY: test
test:
	$(CC) test

.PHONY: install
install:
	$(CC) install --path=./

.PHONY: uninstall
uninstall:
	$(CC) uninstall $(PROGRAM)

.PHONY: systemd-install
systemd-install:
	cp $(SYSTEMD_INSTALL_SRC) $(SYSTEMD_INSTALL_DST)
	cp $(SERVICE_FILE_SRC) $(SERVICE_FILE_DEST)
	cp $(ETC_FILE_SRC) $(ETC_FILE_DST)

.PHONY: systemd-uninstall
systemd-uninstall:
	rm $(SYSTEMD_INSTALL_DST)
	rm $(SERVICE_FILE_DEST)
	rm $(ETC_FILE_DST)
