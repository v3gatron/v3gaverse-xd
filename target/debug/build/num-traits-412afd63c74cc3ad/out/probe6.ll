; ModuleID = 'probe6.2276cced86718871-cgu.0'
source_filename = "probe6.2276cced86718871-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_b6a56228728ac33d9325bcdb6e8fab8a = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/b628260df0587ae559253d8640ecb8738d3de613/library/core/src/num/mod.rs" }>, align 1
@alloc_002375cb7750ee553aaf9500956a5edf = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b6a56228728ac33d9325bcdb6e8fab8a, [16 x i8] c"K\00\00\00\00\00\00\00@\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: uwtable
define void @_ZN6probe65probe17h07007742d6134750E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h5a81f2b1b85b7109E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h5195b0f832713fc2E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_002375cb7750ee553aaf9500956a5edf) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h5a81f2b1b85b7109E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h5195b0f832713fc2E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
