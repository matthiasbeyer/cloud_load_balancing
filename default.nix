{ pkgs ? (import <nixpkgs> {}) }:

let
  tex = (pkgs.texlive.combine {
    inherit (pkgs.texlive)
      scheme-small
      algorithms
      cm-super
      collection-basic
      collection-bibtexextra
      collection-fontsextra
      collection-fontutils
      collection-langenglish
      collection-langgerman
      collection-latex
      collection-latexextra
      collection-latexrecommended
      collection-mathextra
      collection-metapost
      collection-pictures
      collection-plainextra
      collection-science
      tracklang
      glossaries
      luatex
      IEEEtran

      minted
    ;
  });
in
pkgs.stdenv.mkDerivation rec {
    name = "tex";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ tex pkgs.pythonPackages.pygments ];
}
