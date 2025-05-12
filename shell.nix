{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
  packages = with pkgs; [
    rustup
    grub2
    qemu_full
    libisoburn
  ];
}
