Name:           greetme
Version:        1.0.0
Release:        1%{?dist}
Summary:        A fast, themeable terminal greeting application

License:        MIT
URL:            https://github.com/techytim-tech/greetme
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  gcc
Requires:       glibc

%description
greetme is a fast, themeable terminal greeting application with support
for ASCII art fonts and customizable color themes. It follows XDG Base
Directory specification and works across Linux distributions.

%prep
%setup -q

%build
# Binary is pre-built by Cargo

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/greetme/{themes,fonts,examples}
mkdir -p %{buildroot}%{_mandir}/man1
mkdir -p %{buildroot}%{_docdir}/greetme

install -m 0755 greetme %{buildroot}%{_bindir}/greetme
install -m 0644 themes/* %{buildroot}%{_datadir}/greetme/themes/
install -m 0644 fonts/* %{buildroot}%{_datadir}/greetme/fonts/
install -m 0644 examples/* %{buildroot}%{_datadir}/greetme/examples/
install -m 0644 greetme.1.gz %{buildroot}%{_mandir}/man1/
install -m 0644 README.md LICENSE %{buildroot}%{_docdir}/greetme/

%files
%{_bindir}/greetme
%{_datadir}/greetme/
%{_mandir}/man1/greetme.1.gz
%{_docdir}/greetme/

%doc README.md
%license LICENSE

%changelog
* Wed Oct 22 2025 greetme contributors <greetme@example.com> - 1.0.0-1
- Initial RPM release
- Support for 5 built-in themes
- ASCII art rendering with multiple fonts
- XDG Base Directory compliance
