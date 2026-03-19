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
    signal enable: std_logic := '0';
    signal data_in: std_logic_vector(1599 downto 0) := (others => '0');
    signal data_out: std_logic_vector(1599 downto 0);
    signal done: std_logic;

begin
    clk <= not clk after 1 ns;

    Keccak_f_inst: entity work.Keccak_f
     generic map(
        L => 6
    )
     port map(
        clk => clk,
        rst => rst,
        enable => enable,
        data_in => data_in,
        data_out => data_out,
        done => done
    );

    testing: process is
    begin
        rst <= '0';
        enable <= '1';
        --data <=
        --data <= x"10050350350355030553ABED23030303443434" ;
        wait until rising_edge(done);
        wait for 10 ns;

        report "Tests Complete";

        finish;
    end process testing;

end Keccak_f_tb;
