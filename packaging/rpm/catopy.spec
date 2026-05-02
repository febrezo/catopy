Name:           catopy
Version:        0.2.1
Release:        1%{?dist}
Summary:        Copy file content to clipboard safely from the terminal
License:        GPLv3
URL:            https://github.com/febrezo/catopy
Source0:        https://crates.io/api/v1/crates/%{name}/%{version}/download#/%{name}-%{version}.crate

BuildRequires:  rust-packaging
BuildRequires:  cargo

%description
catopy is a Rust CLI utility that copies file content to the clipboard with
size guards and optional head/tail slicing.

%prep
%autosetup -n %{name}-%{version}

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
install -Dm0755 target/release/catopy %{buildroot}%{_bindir}/catopy

install -Dm0644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm0644 CHANGELOG.md %{buildroot}%{_docdir}/%{name}/CHANGELOG.md
install -Dm0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

install -Dm0644 man/catopy.1 %{buildroot}%{_mandir}/man1/catopy.1
gzip -9 %{buildroot}%{_mandir}/man1/catopy.1

install -Dm0644 completions/catopy.bash %{buildroot}%{_datadir}/bash-completion/completions/catopy
install -Dm0644 completions/catopy.zsh %{buildroot}%{_datadir}/zsh/site-functions/_catopy
install -Dm0644 completions/catopy.fish %{buildroot}%{_datadir}/fish/vendor_completions.d/catopy.fish

%files
%license %{_licensedir}/%{name}/LICENSE
%doc %{_docdir}/%{name}/README.md
%doc %{_docdir}/%{name}/CHANGELOG.md
%{_bindir}/catopy
%{_mandir}/man1/catopy.1.gz
%{_datadir}/bash-completion/completions/catopy
%{_datadir}/zsh/site-functions/_catopy
%{_datadir}/fish/vendor_completions.d/catopy.fish

%changelog
* Sat May 03 2026 Felix <felix@example.com> - 0.2.1-1
- Initial RPM packaging skeleton for COPR/Fedora.
