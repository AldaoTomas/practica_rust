; ModuleID = 'probe1.71cb626e88d129ad-cgu.0'
source_filename = "probe1.71cb626e88d129ad-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_C3A54530AEDA3685u = comdat any

$__llvm_profile_filename = comdat any

@alloc_f93507f8ba4b5780b14b2c2584609be0 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc_ef0a1f828f3393ef691f2705e817091c = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [97 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 97, i32 0, i32 5 }, [97 x i8] c"\02^\00VC:\\Users\\Equipo\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.19\06<anon>" }, section ".lcovmap$M", align 8
@__covrec_C3A54530AEDA3685u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -4348993789762062715, i32 9, i64 8718831616378132673, i64 2125308340048687084, [9 x i8] c"\01\01\00\01\01\01\01\002" }>, section ".lcovfun$M", comdat, align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCs9LIY2Fi01pp_6probe15probe = private global [1 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCs9LIY2Fi01pp_6probe15probe = private global { i64, i64, i64, i64, ptr, ptr, i32, [2 x i16], i32 } { i64 -4348993789762062715, i64 8718831616378132673, i64 sub (i64 ptrtoint (ptr @__profc__RNvCs9LIY2Fi01pp_6probe15probe to i64), i64 ptrtoint (ptr @__profd__RNvCs9LIY2Fi01pp_6probe15probe to i64)), i64 0, ptr null, ptr null, i32 1, [2 x i16] zeroinitializer, i32 0 }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCs9LIY2Fi01pp_6probe15probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCs9LIY2Fi01pp_6probe15probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__llvm_coverage_mapping, ptr @__covrec_C3A54530AEDA3685u, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = hidden constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; <f64>::total_cmp
; Function Attrs: inlinehint uwtable
define internal i8 @_RNvMNtCsRCAB2vc60q_4core3f64d9total_cmpCs9LIY2Fi01pp_6probe1(ptr align 8 %self, ptr align 8 %other) unnamed_addr #0 !dbg !17 {
start:
  %other.dbg.spill6 = alloca ptr, align 8
  %self.dbg.spill5 = alloca ptr, align 8
  %self.dbg.spill4 = alloca double, align 8
  %self.dbg.spill2 = alloca double, align 8
  %other.dbg.spill = alloca ptr, align 8
  %self.dbg.spill = alloca ptr, align 8
  %right = alloca i64, align 8
  %left = alloca i64, align 8
  %_0 = alloca i8, align 1
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !27, metadata !DIExpression()), !dbg !36
  store ptr %other, ptr %other.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill, metadata !28, metadata !DIExpression()), !dbg !36
  call void @llvm.dbg.declare(metadata ptr %left, metadata !29, metadata !DIExpression()), !dbg !37
  call void @llvm.dbg.declare(metadata ptr %right, metadata !33, metadata !DIExpression()), !dbg !38
  %self1 = load double, ptr %self, align 8, !dbg !39, !noundef !35
  store double %self1, ptr %self.dbg.spill2, align 8, !dbg !39
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !40, metadata !DIExpression()), !dbg !50
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !51, metadata !DIExpression()), !dbg !58
  %_4 = bitcast double %self1 to i64, !dbg !60
  store i64 %_4, ptr %left, align 8, !dbg !39
  %self3 = load double, ptr %other, align 8, !dbg !61, !noundef !35
  store double %self3, ptr %self.dbg.spill4, align 8, !dbg !61
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !48, metadata !DIExpression()), !dbg !62
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !56, metadata !DIExpression()), !dbg !63
  %_7 = bitcast double %self3 to i64, !dbg !65
  store i64 %_7, ptr %right, align 8, !dbg !61
  %_13 = load i64, ptr %left, align 8, !dbg !66, !noundef !35
  %_12 = ashr i64 %_13, 63, !dbg !66
  %_10 = lshr i64 %_12, 1, !dbg !66
  %0 = load i64, ptr %left, align 8, !dbg !66, !noundef !35
  %1 = xor i64 %0, %_10, !dbg !66
  store i64 %1, ptr %left, align 8, !dbg !66
  %_18 = load i64, ptr %right, align 8, !dbg !67, !noundef !35
  %_17 = ashr i64 %_18, 63, !dbg !67
  %_15 = lshr i64 %_17, 1, !dbg !67
  %2 = load i64, ptr %right, align 8, !dbg !67, !noundef !35
  %3 = xor i64 %2, %_15, !dbg !67
  store i64 %3, ptr %right, align 8, !dbg !67
  store ptr %left, ptr %self.dbg.spill5, align 8, !dbg !68
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill5, metadata !69, metadata !DIExpression()), !dbg !80
  store ptr %right, ptr %other.dbg.spill6, align 8, !dbg !68
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill6, metadata !79, metadata !DIExpression()), !dbg !80
  %_22 = load i64, ptr %left, align 8, !dbg !80, !noundef !35
  %_23 = load i64, ptr %right, align 8, !dbg !80, !noundef !35
  %_21 = icmp slt i64 %_22, %_23, !dbg !80
  br i1 %_21, label %bb1, label %bb2, !dbg !80

bb2:                                              ; preds = %start
  %_24 = icmp eq i64 %_22, %_23, !dbg !80
  br i1 %_24, label %bb3, label %bb4, !dbg !80

bb1:                                              ; preds = %start
  store i8 -1, ptr %_0, align 1, !dbg !80
  br label %bb6, !dbg !80

bb4:                                              ; preds = %bb2
  store i8 1, ptr %_0, align 1, !dbg !80
  br label %bb5, !dbg !80

bb3:                                              ; preds = %bb2
  store i8 0, ptr %_0, align 1, !dbg !80
  br label %bb5, !dbg !80

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6, !dbg !80

bb6:                                              ; preds = %bb1, %bb5
  %4 = load i8, ptr %_0, align 1, !dbg !81, !range !82, !noundef !35
  ret i8 %4, !dbg !81
}

; probe1::probe
; Function Attrs: uwtable
define void @_RNvCs9LIY2Fi01pp_6probe15probe() unnamed_addr #1 !dbg !83 {
start:
  %0 = atomicrmw add ptr @__profc__RNvCs9LIY2Fi01pp_6probe15probe, i64 1 monotonic, align 8, !dbg !88
; call <f64>::total_cmp
  %_1 = call i8 @_RNvMNtCsRCAB2vc60q_4core3f64d9total_cmpCs9LIY2Fi01pp_6probe1(ptr align 8 @alloc_f93507f8ba4b5780b14b2c2584609be0, ptr align 8 @alloc_ef0a1f828f3393ef691f2705e817091c), !dbg !88, !range !82
  ret void, !dbg !89
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #3

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #4 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #1 = { uwtable "target-cpu"="x86-64" "target-features"="+cx16,+sse3,+sahf" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nounwind }
attributes #4 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = !{!"rustc version 1.78.0 (9b00956e5 2024-04-29)"}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.78.0 (9b00956e5 2024-04-29))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !6, splitDebugInlining: false)
!5 = !DIFile(filename: "probe1\\@\\probe1.71cb626e88d129ad-cgu.0", directory: "C:\\Users\\Equipo\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.19")
!6 = !{!7}
!7 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Ordering", scope: !9, file: !8, baseType: !11, size: 8, align: 8, flags: DIFlagEnumClass, elements: !13)
!8 = !DIFile(filename: "<unknown>", directory: "")
!9 = !DINamespace(name: "cmp", scope: !10)
!10 = !DINamespace(name: "core", scope: null)
!11 = !DIDerivedType(tag: DW_TAG_typedef, name: "i8", file: !8, baseType: !12)
!12 = !DIBasicType(name: "__int8", size: 8, encoding: DW_ATE_signed)
!13 = !{!14, !15, !16}
!14 = !DIEnumerator(name: "Less", value: -1)
!15 = !DIEnumerator(name: "Equal", value: 0)
!16 = !DIEnumerator(name: "Greater", value: 1)
!17 = distinct !DISubprogram(name: "total_cmp", linkageName: "_RNvMNtCsRCAB2vc60q_4core3f64d9total_cmpCs9LIY2Fi01pp_6probe1", scope: !19, file: !18, line: 1462, type: !21, scopeLine: 1462, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !35, retainedNodes: !26)
!18 = !DIFile(filename: "/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\\library\\core\\src\\num\\f64.rs", directory: "", checksumkind: CSK_SHA256, checksum: "32dcb646544d701257a45eedb75e432e5ddc030dbbbff6370cb21baa20931e48")
!19 = !DINamespace(name: "impl$0", scope: !20)
!20 = !DINamespace(name: "f64", scope: !10)
!21 = !DISubroutineType(types: !22)
!22 = !{!7, !23, !23}
!23 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref$<f64>", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 0)
!24 = !DIDerivedType(tag: DW_TAG_typedef, name: "f64", file: !8, baseType: !25)
!25 = !DIBasicType(name: "double", size: 64, encoding: DW_ATE_float)
!26 = !{!27, !28, !29, !33}
!27 = !DILocalVariable(name: "self", arg: 1, scope: !17, file: !18, line: 1462, type: !23)
!28 = !DILocalVariable(name: "other", arg: 2, scope: !17, file: !18, line: 1462, type: !23)
!29 = !DILocalVariable(name: "left", scope: !30, file: !18, line: 1463, type: !31, align: 8)
!30 = distinct !DILexicalBlock(scope: !17, file: !18, line: 1463)
!31 = !DIDerivedType(tag: DW_TAG_typedef, name: "i64", file: !8, baseType: !32)
!32 = !DIBasicType(name: "__int64", size: 64, encoding: DW_ATE_signed)
!33 = !DILocalVariable(name: "right", scope: !34, file: !18, line: 1464, type: !31, align: 8)
!34 = distinct !DILexicalBlock(scope: !30, file: !18, line: 1464)
!35 = !{}
!36 = !DILocation(line: 1462, scope: !17)
!37 = !DILocation(line: 1463, scope: !30)
!38 = !DILocation(line: 1464, scope: !34)
!39 = !DILocation(line: 1463, scope: !17)
!40 = !DILocalVariable(name: "self", arg: 1, scope: !41, file: !18, line: 1121, type: !24)
!41 = distinct !DILexicalBlock(scope: !42, file: !18, line: 1121)
!42 = distinct !DISubprogram(name: "to_bits", linkageName: "_RNvMNtCsRCAB2vc60q_4core3f64d7to_bitsCs9LIY2Fi01pp_6probe1", scope: !19, file: !18, line: 1121, type: !43, scopeLine: 1121, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !35, retainedNodes: !47)
!43 = !DISubroutineType(types: !44)
!44 = !{!45, !24}
!45 = !DIDerivedType(tag: DW_TAG_typedef, name: "u64", file: !8, baseType: !46)
!46 = !DIBasicType(name: "unsigned __int64", size: 64, encoding: DW_ATE_unsigned)
!47 = !{!40, !48}
!48 = !DILocalVariable(name: "self", arg: 1, scope: !49, file: !18, line: 1121, type: !24)
!49 = distinct !DILexicalBlock(scope: !42, file: !18, line: 1121)
!50 = !DILocation(line: 1121, scope: !41, inlinedAt: !39)
!51 = !DILocalVariable(name: "rt", arg: 1, scope: !52, file: !18, line: 1143, type: !24)
!52 = distinct !DILexicalBlock(scope: !53, file: !18, line: 1143)
!53 = distinct !DISubprogram(name: "rt_f64_to_u64", linkageName: "_ZN4core3f6421_$LT$impl$u20$f64$GT$7to_bits13rt_f64_to_u6417h364b93e21a6d8976E", scope: !54, file: !18, line: 1143, type: !43, scopeLine: 1143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !35, retainedNodes: !55)
!54 = !DINamespace(name: "to_bits", scope: !19)
!55 = !{!51, !56}
!56 = !DILocalVariable(name: "rt", arg: 1, scope: !57, file: !18, line: 1143, type: !24)
!57 = distinct !DILexicalBlock(scope: !53, file: !18, line: 1143)
!58 = !DILocation(line: 1143, scope: !52, inlinedAt: !59)
!59 = !DILocation(line: 1152, scope: !41, inlinedAt: !39)
!60 = !DILocation(line: 1147, scope: !52, inlinedAt: !59)
!61 = !DILocation(line: 1464, scope: !30)
!62 = !DILocation(line: 1121, scope: !49, inlinedAt: !61)
!63 = !DILocation(line: 1143, scope: !57, inlinedAt: !64)
!64 = !DILocation(line: 1152, scope: !49, inlinedAt: !61)
!65 = !DILocation(line: 1147, scope: !57, inlinedAt: !64)
!66 = !DILocation(line: 1488, scope: !34)
!67 = !DILocation(line: 1489, scope: !34)
!68 = !DILocation(line: 1491, scope: !34)
!69 = !DILocalVariable(name: "self", arg: 1, scope: !70, file: !71, line: 1617, type: !77)
!70 = distinct !DILexicalBlock(scope: !72, file: !71, line: 1565)
!71 = !DIFile(filename: "/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\\library\\core\\src\\cmp.rs", directory: "", checksumkind: CSK_SHA256, checksum: "5f073c59b809df5aa648085799979b2f83825aa3a4363bc2e2794a52e9f5221f")
!72 = distinct !DISubprogram(name: "cmp", linkageName: "_RNvXs1b_NtNtCsRCAB2vc60q_4core3cmp5implsxNtB8_3Ord3cmpCs9LIY2Fi01pp_6probe1", scope: !73, file: !71, line: 1565, type: !75, scopeLine: 1565, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !35, retainedNodes: !78)
!73 = !DINamespace(name: "impl$75", scope: !74)
!74 = !DINamespace(name: "impls", scope: !9)
!75 = !DISubroutineType(types: !76)
!76 = !{!7, !77, !77}
!77 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref$<i64>", baseType: !31, size: 64, align: 64, dwarfAddressSpace: 0)
!78 = !{!69, !79}
!79 = !DILocalVariable(name: "other", arg: 2, scope: !70, file: !71, line: 1617, type: !77)
!80 = !DILocation(line: 1617, scope: !70, inlinedAt: !68)
!81 = !DILocation(line: 1492, scope: !17)
!82 = !{i8 -1, i8 2}
!83 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCs9LIY2Fi01pp_6probe15probe", scope: !85, file: !84, line: 1, type: !86, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !35)
!84 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA256, checksum: "4413f23fc88f80784bc38c0596e470b2a62bd5c29edf8d750709442bb82c9abb")
!85 = !DINamespace(name: "probe1", scope: null)
!86 = !DISubroutineType(types: !87)
!87 = !{null}
!88 = !DILocation(line: 1, scope: !83)
!89 = !DILocation(line: 1, scope: !90)
!90 = !DILexicalBlockFile(scope: !83, file: !84, discriminator: 0)
