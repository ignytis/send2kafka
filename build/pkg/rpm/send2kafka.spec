Name:       send2kafka
Version:    {{ VERSION }}
Release:    1
Summary:    HTTP interface for Kafka
License:    GPL-3.0

Requires: zlib >= 1.2.0, zlib < 2.0.0, cyrus-sasl-lib >= 2.1.0, cyrus-sasl-lib < 3.0.0, openssl-libs >= 1:3.1.0, openssl-libs < 1:4.0.0, libgcc >= 13.2.0, libgcc < 14.0.0, glibc >= 2.35, glibc < 3, libxcrypt >= 4.4.0, libxcrypt < 5.0.0

%description
A HTTP API service which forwards the HTTP requests to Kafka broker

%install
mkdir -p %{buildroot}/usr/local/bin/
install -m 755 send2kafka %{buildroot}/usr/local/bin/send2kafka
 
%files                                                   
/usr/local/bin/send2kafka

%changelog

* Tue Apr 16 2024 Ignytis <155588001+ignytis@users.noreply.github.com> - 1.0.0-0
- Initial RPM release
