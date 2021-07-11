CC	= cargo
PROD_FLAGS	=	--release
PROGRAM	=	img_sync
SERVICE_FILE_DEST	=	/etc/systemd/system/img_sync.service
SERVICE_FILE_SRC	=	./etc/img_sync.service

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
systemd-install: install
	cp $(SERVICE_FILE_SRC) $(SERVICE_FILE_DEST)

.PHONY: systemd-uninstall
systemd-uninstall: uninstall
	rm $(SERVICE_FILE_DEST)
