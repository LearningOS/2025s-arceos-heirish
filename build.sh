#!/bin/bash
set -x
cp config.toml /root/.cargo/

run_print_tutor() {
make run
}
run_print_exercise() {
make run A=exercises/print_woth_color
}

run_alloc_tutor() {
make run A=tour/u_2_0
}
run_hashmap_exercise() {
make run A=exercises/support_hashmap
}
run_pflash_mm_tutor() {
make run A=tour/u_3_0
}
run_alloc_exercise() {
make run A=exercises/alt_alloc
}
run_multitask_tutor() {
make run A=tour/u_4_0
}
run_multitask_comm_tutor() {
make run A=tour/u_5_0 LOG=trace
}


run_taskscheduler_tutor() {
make run A=tour/u_6_0
}
run_taskscheduler_tutor1() {
make run A=tour/u_6_1
}
run_blkdriver_tutor() {
make run A=tour/u_7_0 BLK=y LOG=trace
}
run_fs_tutor() {
make run A=tour/u_8_0 BLK=y
}
run_fs_ex() {
make run A=examples/shell BLK=y
}
run_fs_ex1() {
make run A=exercises/ramfs_rename BLK=y LOG=trace
}

run_monolithic_kernel_uapp_tutor() {
make payload
./update_disk.sh ./payload/origin/origin
make run A=tour/m_1_0 BLK=y
}
run_monolithic_kernel_uapp_tutor2() {
make payload
./update_disk.sh ./payload/origin/origin
make run A=tour/m_2_0 BLK=y
}
run_mmap_ex() {
make payload
./update_disk.sh ./payload/mapfile_c/mapfile
make run A=exercises/sys_map BLK=y
}

run_mono_musl_userapp() {
make payload
./update_disk.sh ./payload/hello_c/hello
make run A=tour/m_3_0 BLK=y
}

cd arceos
#make clean
make pflash_img
make disk_img

####lesson 1
#run_print_tutor
#run_print_exercise

######lesson 2
#run_alloc_tutor
#run_pflash_mm_tutor
#run_hashmap_exercise
#run_alloc_exercise
#run_multitask_tutor
#run_multitask_comm_tutor

########lesson 3
#run_taskscheduler_tutor
#run_taskscheduler_tutor1
#run_blkdriver_tutor
#run_fs_ex
run_fs_ex1

########lesson 4 monolithic uspace
#run_monolithic_kernel_uapp_tutor
#run_monolithic_kernel_uapp_tutor2
#run_mmap_ex


#### lesson5 monolithic support musl-libc userapp
run_mono_musl_userapp
