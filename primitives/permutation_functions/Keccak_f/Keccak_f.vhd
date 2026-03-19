library ieee;
use ieee.std_logic_1164.all;
use IEEE.NUMERIC_STD.ALL;
use IEEE.MATH_REAL.ALL;

entity Keccak_f is
    generic (
        WIDTH: natural
    );
    port (
        in_bit: in std_logic;
        out_bit: out std_logic
    );
end Keccak_f;

architecture Keccak_f of Keccak_f is

begin
    out_bit <= in_bit;
end Keccak_f;
