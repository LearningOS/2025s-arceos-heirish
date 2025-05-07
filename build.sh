#!/bin/bash
set -x

run_tutor1() {
cd arceos
make pflash_img
make disk_img
make run
}

run_tutor2() {
cd arceos
make pflash_img
make disk_img
make run A=tour/u_2_0
}

run_ex2() {
cd arceos
make clean
make pflash_img
make disk_img
make run A=exercises/support_hashmap
}

#run_tutor2
run_ex2
