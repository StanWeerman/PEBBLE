library ieee;
use ieee.std_logic_1164.all;
use IEEE.NUMERIC_STD.ALL;
use IEEE.MATH_REAL.ALL;

entity Keccak_f is
    generic (
        L: natural
    );
    port (
        clk, rst, enable: in std_logic;
        data_in: in std_logic_vector((25 * (2 ** L))-1 downto 0);
        data_out: in std_logic_vector((25 * (2 ** L))-1 downto 0);
        done: out std_logic
    );
end Keccak_f;

architecture Keccak_f of Keccak_f is
    constant B: natural := 25 * (2 ** L);
    constant W: natural := 2 ** L;
    constant ROUND_NUM: natural := 12 + (2*L);
    signal round_count: natural range 0 to ROUND_NUM - 1;

    type r_array is array (0 to 4, 0 to 4) of integer;
    constant R: r_array := (
                        (0,1,62,28,27),
                        (36,44,6,55,20),
                        (3,10,43,25,39),
                        (41,45,15,21,8),
                        (18,2,61,56,14)
                        );

    type a_array is array (0 to 4, 0 to 4) of std_logic_vector(W-1 downto 0);
    signal A: a_array;

    type rc_constant_array is array(0 to 23) of std_logic_vector(63 downto 0);
    constant rc_constant: rc_constant_array := (
                                                x"0000000000000001",
                                                x"0000000000008082",
                                                x"800000000000808A",
                                                x"8000000080008000",
                                                x"000000000000808B",
                                                x"0000000080000001",
                                                x"8000000080008081",
                                                x"8000000000008009",
                                                x"000000000000008A",
                                                x"0000000000000088",
                                                x"0000000080008009",
                                                x"000000008000000A",
                                                x"000000008000808B",
                                                x"800000000000008B",
                                                x"8000000000008089",
                                                x"8000000000008003",
                                                x"8000000000008002",
                                                x"8000000000000080",
                                                x"000000000000800A",
                                                x"800000008000000A",
                                                x"8000000080008081",
                                                x"8000000000008080",
                                                x"0000000080000001",
                                                x"8000000080008008"
                                                );

    type rc_array is array(0 to ROUND_NUM-1) of std_logic_vector(W-1 downto 0);
    function init_rc return rc_array is
        variable temp_rc: rc_array;
    begin
            --temp_rc := (others => (others => (others => (others => '0'))));
            --for i in 0 to ROUND_NUM-1 loop
            --    for j in 0 to L-1 loop
            --        temp_rc(i)(0,0)((2**j)-1) :=  rc_constant(j+7*i);
            --    end loop;
            --end loop;
            for i in 0 to ROUND_NUM-1 loop
                temp_rc(i) := rc_constant(i);
            end loop;
            return temp_rc;
    end function;
    constant RC: rc_array := init_rc;

    --signal B;
    signal C, D: std_logic_vector(4 downto 0);
begin

    round: process (clk) is
        variable C, D: std_logic_vector(4 downto 0);
        variable A_temp: a_array;
        variable B: a_array;
        variable pi_1, pi_2: std_logic_vector(W-1 downto 0);
        variable z_index_minus_1, x_index_minus_1, x_index_plus_1, x_index_plus_2: natural;
        variable b_index: natural;
    begin
        if (rst) then
            round_count <= 0;
            done <= '0';
        elsif (rising_edge(clk)) then

        -- Round
            -- Theta
            for z in 0 to W-1 loop
                for x in 0 to 4 loop
                    for y in 0 to 4 loop
                        x_index_plus_1 := 0 when x = 4 else x+1;
                        x_index_minus_1 := 4 when x = 0 else x-1;
                        z_index_minus_1 := W-1 when z = 0 else z-1;
                        pi_1(z) := a(x_index_minus_1,0)(z) xor a(x_index_minus_1,1)(z) xor a(x_index_minus_1,2)(z) xor a(x_index_minus_1,3)(z) xor a(x_index_minus_1,4)(z);
                        pi_2(z) := a(x_index_plus_1,0)(z_index_minus_1) xor a(x_index_plus_1,1)(z_index_minus_1) xor a(x_index_plus_1,2)(z_index_minus_1) xor a(x_index_plus_1,3)(z_index_minus_1) xor a(x_index_plus_1,4)(z_index_minus_1);
                        A_temp(x,y)(z) := a(x,y)(z);
                    end loop;
                end loop;
            end loop;

            -- Rho
            for x in 0 to 4 loop
                for y in 0 to 4 loop
                    b_index := (2*x+3*y) mod 4;
                    B(y, b_index) := a(x,y) ror r(x,y);
                end loop;
            end loop;

            -- Pi

            -- Chi
            for x in 0 to 4 loop
                for y in 0 to 4 loop
                    x_index_plus_1 := 0 when x = 4 else x+1;
                    x_index_plus_2 := 0 when x_index_plus_1 = 4 else x_index_plus_1+1;
                    A_temp(x,y) := B(x,y) xor (not b(x_index_plus_1,y) and b(x_index_plus_2,y));
                end loop;
            end loop;

            --Lambda
            --for x in 0 to 4 loop
            --    for y in 0 to 4 loop
                    A(0,0) <= A(0,0) xor RC(round_count);
                    if round_count = ROUND_NUM-1 then
                        round_count <= 0;
                        done <= '1';
                    else round_count <= round_count + 1;
                    end if;
            --    end loop;
            --end loop;
        end if;
    end process round;

    --assign_data: process (all) is
    --begin

    --end process assign_data;

end Keccak_f;
