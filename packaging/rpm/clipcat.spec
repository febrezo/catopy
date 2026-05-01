Name:           clipcat
Version:        0.1.0
Release:        1%{?dist}
Summary:        Copy file content to clipboard safely from the terminal
License:        GPLv3
URL:            https://github.com/felix/clipcat
Source0:        https://crates.io/api/v1/crates/%{name}/%{version}/download#/%{name}-%{version}.crate

BuildRequires:  rust-packaging
BuildRequires:  cargo

%description
clipcat is a Rust CLI utility that copies file content to the clipboard with
size guards and optional head/tail slicing.

%prep
%autosetup -n %{name}-%{version}

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build --release

%install
install -Dm0755 target/release/clipcat %{buildroot}%{_bindir}/clipcat

install -Dm0644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm0644 CHANGELOG.md %{buildroot}%{_docdir}/%{name}/CHANGELOG.md
install -Dm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

install -Dm0644 man/clipcat.1 %{buildroot}%{_mandir}/man1/clipcat.1
gzip -9 %{buildroot}%{_mandir}/man1/clipcat.1

install -Dm0644 completions/clipcat.bash %{buildroot}%{_datadir}/bash-completion/completions/clipcat
install -Dm0644 completions/clipcat.zsh %{buildroot}%{_datadir}/zsh/site-functions/_clipcat
install -Dm0644 completions/clipcat.fish %{buildroot}%{_datadir}/fish/vendor_completions.d/clipcat.fish

%files
%license %{_licensedir}/%{name}/LICENSE
%doc %{_docdir}/%{name}/README.md
%doc %{_docdir}/%{name}/CHANGELOG.md
%{_bindir}/clipcat
%{_mandir}/man1/clipcat.1.gz
%{_datadir}/bash-completion/completions/clipcat
%{_datadir}/zsh/site-functions/_clipcat
%{_datadir}/fish/vendor_completions.d/clipcat.fish

%changelog
* Thu May 01 2026 Felix <felix@example.com> - 0.1.0-1
- Initial RPM packaging skeleton for COPR/Fedora.
