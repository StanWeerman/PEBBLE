library ieee;
use ieee.std_logic_1164.all;
use std.env.finish;
use IEEE.NUMERIC_STD.ALL;
use IEEE.MATH_REAL.ALL;

entity Keccak_f_tb is
end Keccak_f_tb;

architecture Keccak_f_tb of Keccak_f_tb is
    signal clk: std_logic := '0';
    signal rst: std_logic := '1';
    signal in_bit, out_bit: std_logic;

begin
    clk <= not clk after 1 ns;

    Keccak_f_inst: entity work.Keccak_f
     generic map(
        WIDTH => 5
    )
     port map(
        in_bit => in_bit,
        out_bit => out_bit
    );

    testing: process is
    begin
        in_bit <= '0';
        wait for 1 ns;
        in_bit <= '1';
        wait for 1 ns;
        in_bit <= '0';

        report "Tests Complete";

        finish;
    end process testing;

end Keccak_f_tb;
