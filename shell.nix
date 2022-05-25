# https://github.com/target/lorri/issues/460#issuecomment-870901956
{ system ? builtins.currentSystem }:
(builtins.getFlake (toString ./.)).devShell.${system}
