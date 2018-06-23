    7da4:	66 0f ef c0          	pxor   %xmm0,%xmm0
    7da8:	66 0f 7f 84 24 80 00 	movdqa %xmm0,0x80(%rsp)
    7daf:	00 00 
    7db1:	66 0f 7f 44 24 70    	movdqa %xmm0,0x70(%rsp)
    7db7:	66 0f 7f 44 24 60    	movdqa %xmm0,0x60(%rsp)
    7dbd:	66 0f 7f 44 24 50    	movdqa %xmm0,0x50(%rsp)
    7dc3:	66 0f 7f 44 24 40    	movdqa %xmm0,0x40(%rsp)
    7dc9:	66 0f 7f 44 24 30    	movdqa %xmm0,0x30(%rsp)
    7dcf:	66 0f 7f 44 24 20    	movdqa %xmm0,0x20(%rsp)
    7dd5:	66 0f 7f 44 24 10    	movdqa %xmm0,0x10(%rsp)
    7ddb:	66 0f 7f 04 24       	movdqa %xmm0,(%rsp)
    7de0:	e8 eb 1e 00 00       	callq  9cd0 <_ZN4rand4rngs6thread10thread_rng17h696855ef8b920c71E>
    7de5:	48 89 84 24 e0 00 00 	mov    %rax,0xe0(%rsp)      ;thread_rng
    7dec:	00 
    7ded:	48 89 e7             	mov    %rsp,%rdi
    7df0:	be 24 00 00 00       	mov    $0x24,%esi
    7df5:	e8 c6 2e 00 00       	callq  acc0 <_ZN54_$LT$$u5b$i32$u5d$$u20$as$u20$rand..AsByteSliceMut$GT$17as_byte_slice_mut17h331584227d8ae4f3E>
    7dfa:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi      ;thread_rng
    7e01:	00 
    7e02:	48 89 c6             	mov    %rax,%rsi
    7e05:	e8 d6 1e 00 00       	callq  9ce0 <_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$10fill_bytes17h7bc130caa78626f8E>
    7e0a:	48 89 e7             	mov    %rsp,%rdi
    7e0d:	be 24 00 00 00       	mov    $0x24,%esi
    7e12:	e8 c9 2e 00 00       	callq  ace0 <_ZN54_$LT$$u5b$i32$u5d$$u20$as$u20$rand..AsByteSliceMut$GT$5to_le17h164b872b49833b37E>
    7e17:	48 8b bc 24 e0 00 00 	mov    0xe0(%rsp),%rdi      ;thread_rng
    7e1e:	00 
    7e1f:	48 8b 07             	mov    (%rdi),%rax
    7e22:	48 83 c0 ff          	add    $0xffffffffffffffff,%rax
    7e26:	48 89 07             	mov    %rax,(%rdi)
    7e29:	75 16                	jne    7e41 <_ZN9iterators4main17h38aa83a3e6d38293E+0x8b1>  ;7e41
    7e2b:	48 83 47 08 ff       	addq   $0xffffffffffffffff,0x8(%rdi)
    7e30:	75 0f                	jne    7e41 <_ZN9iterators4main17h38aa83a3e6d38293E+0x8b1>  ;7e41
    7e32:	be 90 10 00 00       	mov    $0x1090,%esi
    7e37:	ba 08 00 00 00       	mov    $0x8,%edx
    7e3c:	e8 9f 19 00 00       	callq  97e0 <__rust_dealloc>    ; drop thread_rng?
    7e41:	48 8d 94 24 90 00 00 	lea    0x90(%rsp),%rdx
    7e48:	00 
    7e49:	48 8d bc 24 90 00 00 	lea    0x90(%rsp),%rdi
    7e50:	00 
    7e51:	48 89 e6             	mov    %rsp,%rsi
    7e54:	e8 47 10 00 00       	callq  8ea0 <_ZN85_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$9from_iter17ha21141cf516d53aeE>
    7e59:	4c 8b a4 24 90 00 00 	mov    0x90(%rsp),%r12
    7e60:	00 
    7e61:	48 8b 9c 24 a0 00 00 	mov    0xa0(%rsp),%rbx
    7e68:	00 
    7e69:	48 8d 0d 03 15 05 00 	lea    0x51503(%rip),%rcx        # 59373 <byte_str.3.llvm.7122942381147639167+0x43>     ; ", "
    7e70:	4c 8d ac 24 c0 00 00 	lea    0xc0(%rsp),%r13
    7e77:	00 
    7e78:	41 b8 02 00 00 00    	mov    $0x2,%r8d
    7e7e:	4c 89 ef             	mov    %r13,%rdi
    7e81:	4c 89 e6             	mov    %r12,%rsi
    7e84:	48 89 da             	mov    %rbx,%rdx
    7e87:	e8 94 11 00 00       	callq  9020 <_ZN5alloc3str81_$LT$impl$u20$alloc..slice..SliceConcatExt$LT$str$GT$$u20$for$u20$$u5b$S$u5d$$GT$4join17h9d4f3f80d6d3918bE>
    7e8c:	4c 89 ac 24 70 01 00 	mov    %r13,0x170(%rsp)
    7e93:	00 
    7e94:	48 8d 05 d5 f6 ff ff 	lea    -0x92b(%rip),%rax        # 7570 <_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd3d99ed8c0ac47caE>
    7e9b:	48 89 84 24 78 01 00 	mov    %rax,0x178(%rsp)
    7ea2:	00 
    7ea3:	48 8d 05 4e 8a 26 00 	lea    0x268a4e(%rip),%rax        # 2708f8 <byte_str.1.llvm.7122942381147639167+0x10>
    7eaa:	48 89 84 24 e0 00 00 	mov    %rax,0xe0(%rsp)
    7eb1:	00 
    7eb2:	48 c7 84 24 e8 00 00 	movq   $0x2,0xe8(%rsp)
    7eb9:	00 02 00 00 00 
    7ebe:	48 8d 05 2b 14 05 00 	lea    0x5142b(%rip),%rax        # 592f0 <byte_str.2.llvm.7122942381147639167>
    7ec5:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
    7ecc:	00 
    7ecd:	48 c7 84 24 f8 00 00 	movq   $0x1,0xf8(%rsp)
    7ed4:	00 01 00 00 00 
    7ed9:	48 8d 84 24 70 01 00 	lea    0x170(%rsp),%rax
    7ee0:	00 
    7ee1:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
    7ee8:	00 
    7ee9:	48 c7 84 24 08 01 00 	movq   $0x1,0x108(%rsp)
    7ef0:	00 01 00 00 00 
    7ef5:	4c 8d ac 24 e0 00 00 	lea    0xe0(%rsp),%r13      ;&(coefficients[0])
    7efc:	00 
    7efd:	4c 89 ef             	mov    %r13,%rdi
    7f00:	e8 0b c7 00 00       	callq  14610 <_ZN3std2io5stdio6_print17h42e700de7189e998E>
    7f05:	48 8b b4 24 c8 00 00 	mov    0xc8(%rsp),%rsi
    7f0c:	00 
    7f0d:	48 85 f6             	test   %rsi,%rsi
    7f10:	74 12                	je     7f24 <_ZN9iterators4main17h38aa83a3e6d38293E+0x994>  ;7f24
    7f12:	48 8b bc 24 c0 00 00 	mov    0xc0(%rsp),%rdi
    7f19:	00 
    7f1a:	ba 01 00 00 00       	mov    $0x1,%edx
    7f1f:	e8 bc 18 00 00       	callq  97e0 <__rust_dealloc>
    7f24:	48 85 db             	test   %rbx,%rbx
    7f27:	74 38                	je     7f61 <_ZN9iterators4main17h38aa83a3e6d38293E+0x9d1>  ;7f61
    7f29:	49 8d 6c 24 08       	lea    0x8(%r12),%rbp
    7f2e:	48 c1 e3 03          	shl    $0x3,%rbx
    7f32:	48 8d 1c 5b          	lea    (%rbx,%rbx,2),%rbx
    7f36:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
    7f3d:	00 00 00 
    7f40:	48 8b 75 00          	mov    0x0(%rbp),%rsi
    7f44:	48 85 f6             	test   %rsi,%rsi
    7f47:	74 0e                	je     7f57 <_ZN9iterators4main17h38aa83a3e6d38293E+0x9c7>  ;7f57
    7f49:	48 8b 7d f8          	mov    -0x8(%rbp),%rdi
    7f4d:	ba 01 00 00 00       	mov    $0x1,%edx
    7f52:	e8 89 18 00 00       	callq  97e0 <__rust_dealloc>    ; drop all the Strings generated in the .map()
    7f57:	48 83 c5 18          	add    $0x18,%rbp
    7f5b:	48 83 c3 e8          	add    $0xffffffffffffffe8,%rbx
    7f5f:	75 df                	jne    7f40 <_ZN9iterators4main17h38aa83a3e6d38293E+0x9b0>  ;7f40
    7f61:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
    7f68:	00 
    7f69:	48 85 c0             	test   %rax,%rax
    7f6c:	74 15                	je     7f83 <_ZN9iterators4main17h38aa83a3e6d38293E+0x9f3>  ;7f83
    7f6e:	48 c1 e0 03          	shl    $0x3,%rax
    7f72:	48 8d 34 40          	lea    (%rax,%rax,2),%rsi
    7f76:	ba 08 00 00 00       	mov    $0x8,%edx
    7f7b:	4c 89 e7             	mov    %r12,%rdi
    7f7e:	e8 5d 18 00 00       	callq  97e0 <__rust_dealloc>
    7f83:	0f 28 05 06 13 05 00 	movaps 0x51306(%rip),%xmm0        # 59290 <_fini+0x84>  ;1,1
    7f8a:	0f 29 84 24 e0 00 00 	movaps %xmm0,0xe0(%rsp)
    7f91:	00 
    7f92:	0f 28 05 07 13 05 00 	movaps 0x51307(%rip),%xmm0        # 592a0 <_fini+0x94>  ;1,2
    7f99:	0f 29 84 24 f0 00 00 	movaps %xmm0,0xf0(%rsp)
    7fa0:	00 
    7fa1:	0f 28 05 08 13 05 00 	movaps 0x51308(%rip),%xmm0        # 592b0 <_fini+0xa4>  ;2,3
    7fa8:	0f 29 84 24 00 01 00 	movaps %xmm0,0x100(%rsp)
    7faf:	00 
    7fb0:	0f 28 05 09 13 05 00 	movaps 0x51309(%rip),%xmm0        # 592c0 <_fini+0xb4>  ;3,4
    7fb7:	0f 29 84 24 10 01 00 	movaps %xmm0,0x110(%rsp)
    7fbe:	00 
    7fbf:	0f 28 05 0a 13 05 00 	movaps 0x5130a(%rip),%xmm0        # 592d0 <_fini+0xc4>  ;4,5
    7fc6:	0f 29 84 24 20 01 00 	movaps %xmm0,0x120(%rsp)
    7fcd:	00 
    7fce:	66 0f 6f 05 0a 13 05 	movdqa 0x5130a(%rip),%xmm0        # 592e0 <_fini+0xd4>  ;5,7
    7fd5:	00 
    7fd6:	66 0f 7f 84 24 30 01 	movdqa %xmm0,0x130(%rsp)
    7fdd:	00 00 
    7fdf:	4c 8d a4 24 40 01 00 	lea    0x140(%rsp),%r12         ;?
    7fe6:	00 
    7fe7:	48 8d 6c 24 30       	lea    0x30(%rsp),%rbp          ;&(buffer[12])
    7fec:	41 be 0c 00 00 00    	mov    $0xc,%r14d               ;12
    7ff2:	48 8d 9c 24 90 00 00 	lea    0x90(%rsp),%rbx          ;?
    7ff9:	00 
    7ffa:	41 bf 0c 00 00 00    	mov    $0xc,%r15d               ;i=12
loop:
    8000:	48 8d 45 d0          	lea    -0x30(%rbp),%rax         ;&(buffer[0])
    8004:	4c 89 ac 24 90 00 00 	mov    %r13,0x90(%rsp)          ;&(coefficients[0])
    800b:	00 
    800c:	4c 89 a4 24 98 00 00 	mov    %r12,0x98(%rsp)          ;?
    8013:	00 
    8014:	48 89 84 24 a0 00 00 	mov    %rax,0xa0(%rsp)          ;&(buffer[0])
    801b:	00 
    801c:	48 89 ac 24 a8 00 00 	mov    %rbp,0xa8(%rsp)          ;&(buffer[12])
    8023:	00 
    8024:	66 49 0f 6e c6       	movq   %r14,%xmm0               ;12
    8029:	66 0f 73 f8 08       	pslldq $0x8,%xmm0               ;12<<(8*8)
    802e:	f3 0f 7f 84 24 b0 00 	movdqu %xmm0,0xb0(%rsp)         ;[0xb0(%rsp): 0x0, 0xb8(%rsp): 0xc]
    8035:	00 00 
    8037:	31 f6                	xor    %esi,%esi
    8039:	48 89 df             	mov    %rbx,%rdi                ;&(coefficients[0])
    803c:	e8 ef 15 00 00       	callq  9630 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E>    ; .map().sum()
    8041:	48 c1 e8 04          	shr    $0x4,%rax                ;>> qlp_shift
    8045:	01 45 00             	add    %eax,0x0(%rbp)           ;buffer[i] += prediction
    8048:	49 83 c7 01          	add    $0x1,%r15                ;i++
    804c:	48 83 c5 04          	add    $0x4,%rbp                ;buffer[i]
    8050:	49 83 ff 24          	cmp    $0x24,%r15               ;i < 36
    8054:	72 aa                	jb     8000 <_ZN9iterators4main17h38aa83a3e6d38293E+0xa70>      ;jb loop
    8056:	48 8d bc 24 90 00 00 	lea    0x90(%rsp),%rdi
    805d:	00 
    805e:	48 89 e6             	mov    %rsp,%rsi
    8061:	48 8d 94 24 90 00 00 	lea    0x90(%rsp),%rdx
    8068:	00 
    8069:	e8 02 0d 00 00       	callq  8d70 <_ZN85_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$9from_iter17h99a8c96100b74849E>
    806e:	4c 8b b4 24 90 00 00 	mov    0x90(%rsp),%r14
    8075:	00 
    8076:	48 8b 9c 24 a0 00 00 	mov    0xa0(%rsp),%rbx
    807d:	00 
    807e:	48 8d 0d ee 12 05 00 	lea    0x512ee(%rip),%rcx        # 59373 <byte_str.3.llvm.7122942381147639167+0x43>
    8085:	4c 8d a4 24 c0 00 00 	lea    0xc0(%rsp),%r12
    808c:	00 
    808d:	41 b8 02 00 00 00    	mov    $0x2,%r8d
    8093:	4c 89 e7             	mov    %r12,%rdi
    8096:	4c 89 f6             	mov    %r14,%rsi
    8099:	48 89 da             	mov    %rbx,%rdx
    809c:	e8 7f 0f 00 00       	callq  9020 <_ZN5alloc3str81_$LT$impl$u20$alloc..slice..SliceConcatExt$LT$str$GT$$u20$for$u20$$u5b$S$u5d$$GT$4join17h9d4f3f80d6d3918bE>
    80a1:	4c 89 a4 24 70 01 00 	mov    %r12,0x170(%rsp)
    80a8:	00 
    80a9:	48 8d 05 c0 f4 ff ff 	lea    -0xb40(%rip),%rax        # 7570 <_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hd3d99ed8c0ac47caE>
    80b0:	48 89 84 24 78 01 00 	mov    %rax,0x178(%rsp)
    80b7:	00 
    80b8:	48 8d 05 59 88 26 00 	lea    0x268859(%rip),%rax        # 270918 <byte_str.1.llvm.7122942381147639167+0x30>
    80bf:	48 89 84 24 e0 00 00 	mov    %rax,0xe0(%rsp)
    80c6:	00 
    80c7:	48 c7 84 24 e8 00 00 	movq   $0x2,0xe8(%rsp)
    80ce:	00 02 00 00 00 
    80d3:	48 8d 05 16 12 05 00 	lea    0x51216(%rip),%rax        # 592f0 <byte_str.2.llvm.7122942381147639167>
    80da:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
    80e1:	00 
    80e2:	48 c7 84 24 f8 00 00 	movq   $0x1,0xf8(%rsp)
    80e9:	00 01 00 00 00 
    80ee:	48 8d 84 24 70 01 00 	lea    0x170(%rsp),%rax
    80f5:	00 
    80f6:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
    80fd:	00 
    80fe:	48 c7 84 24 08 01 00 	movq   $0x1,0x108(%rsp)
    8105:	00 01 00 00 00 
    810a:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
    8111:	00 
    8112:	e8 f9 c4 00 00       	callq  14610 <_ZN3std2io5stdio6_print17h42e700de7189e998E>
    8117:	48 8b b4 24 c8 00 00 	mov    0xc8(%rsp),%rsi
    811e:	00 
    811f:	48 85 f6             	test   %rsi,%rsi
    8122:	74 12                	je     8136 <_ZN9iterators4main17h38aa83a3e6d38293E+0xba6>
    8124:	48 8b bc 24 c0 00 00 	mov    0xc0(%rsp),%rdi
    812b:	00 
    812c:	ba 01 00 00 00       	mov    $0x1,%edx
    8131:	e8 aa 16 00 00       	callq  97e0 <__rust_dealloc>
    8136:	48 85 db             	test   %rbx,%rbx
    8139:	74 36                	je     8171 <_ZN9iterators4main17h38aa83a3e6d38293E+0xbe1>
    813b:	49 8d 6e 08          	lea    0x8(%r14),%rbp
    813f:	48 c1 e3 03          	shl    $0x3,%rbx
    8143:	48 8d 1c 5b          	lea    (%rbx,%rbx,2),%rbx
    8147:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
    814e:	00 00 
    8150:	48 8b 75 00          	mov    0x0(%rbp),%rsi
    8154:	48 85 f6             	test   %rsi,%rsi
    8157:	74 0e                	je     8167 <_ZN9iterators4main17h38aa83a3e6d38293E+0xbd7>
    8159:	48 8b 7d f8          	mov    -0x8(%rbp),%rdi
    815d:	ba 01 00 00 00       	mov    $0x1,%edx
    8162:	e8 79 16 00 00       	callq  97e0 <__rust_dealloc>
    8167:	48 83 c5 18          	add    $0x18,%rbp
    816b:	48 83 c3 e8          	add    $0xffffffffffffffe8,%rbx
    816f:	75 df                	jne    8150 <_ZN9iterators4main17h38aa83a3e6d38293E+0xbc0>
    8171:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
    8178:	00 
    8179:	48 85 c0             	test   %rax,%rax
    817c:	74 15                	je     8193 <_ZN9iterators4main17h38aa83a3e6d38293E+0xc03>
    817e:	48 c1 e0 03          	shl    $0x3,%rax
    8182:	48 8d 34 40          	lea    (%rax,%rax,2),%rsi
    8186:	ba 08 00 00 00       	mov    $0x8,%edx
    818b:	4c 89 f7             	mov    %r14,%rdi
    818e:	e8 4d 16 00 00       	callq  97e0 <__rust_dealloc>
    8193:	48 8b 9c 24 60 01 00 	mov    0x160(%rsp),%rbx
    819a:	00 
    819b:	48 85 db             	test   %rbx,%rbx
    819e:	74 31                	je     81d1 <_ZN9iterators4main17h38aa83a3e6d38293E+0xc41>
    81a0:	48 8b ac 24 50 01 00 	mov    0x150(%rsp),%rbp
    81a7:	00 
    81a8:	48 83 c5 08          	add    $0x8,%rbp
    81ac:	48 c1 e3 05          	shl    $0x5,%rbx
    81b0:	48 8b 75 00          	mov    0x0(%rbp),%rsi
    81b4:	48 85 f6             	test   %rsi,%rsi
    81b7:	74 0e                	je     81c7 <_ZN9iterators4main17h38aa83a3e6d38293E+0xc37>
    81b9:	48 8b 7d f8          	mov    -0x8(%rbp),%rdi
    81bd:	ba 01 00 00 00       	mov    $0x1,%edx
    81c2:	e8 19 16 00 00       	callq  97e0 <__rust_dealloc>
    81c7:	48 83 c5 20          	add    $0x20,%rbp
    81cb:	48 83 c3 e0          	add    $0xffffffffffffffe0,%rbx
    81cf:	75 df                	jne    81b0 <_ZN9iterators4main17h38aa83a3e6d38293E+0xc20>
    81d1:	48 8b b4 24 58 01 00 	mov    0x158(%rsp),%rsi
    81d8:	00 
    81d9:	48 85 f6             	test   %rsi,%rsi
    81dc:	74 16                	je     81f4 <_ZN9iterators4main17h38aa83a3e6d38293E+0xc64>
    81de:	48 8b bc 24 50 01 00 	mov    0x150(%rsp),%rdi
    81e5:	00 
    81e6:	48 c1 e6 05          	shl    $0x5,%rsi
    81ea:	ba 08 00 00 00       	mov    $0x8,%edx
    81ef:	e8 ec 15 00 00       	callq  97e0 <__rust_dealloc>
    81f4:	48 8b b4 24 88 01 00 	mov    0x188(%rsp),%rsi
    81fb:	00 
    81fc:	48 85 f6             	test   %rsi,%rsi
    81ff:	48 8b 9c 24 68 01 00 	mov    0x168(%rsp),%rbx
    8206:	00 
    8207:	74 16                	je     821f <_ZN9iterators4main17h38aa83a3e6d38293E+0xc8f>
    8209:	48 8b bc 24 80 01 00 	mov    0x180(%rsp),%rdi
    8210:	00 
    8211:	48 c1 e6 02          	shl    $0x2,%rsi
    8215:	ba 04 00 00 00       	mov    $0x4,%edx
    821a:	e8 c1 15 00 00       	callq  97e0 <__rust_dealloc>
    821f:	be 0c 00 00 00       	mov    $0xc,%esi
    8224:	ba 04 00 00 00       	mov    $0x4,%edx
    8229:	48 89 df             	mov    %rbx,%rdi
    822c:	e8 af 15 00 00       	callq  97e0 <__rust_dealloc>
    8231:	48 81 c4 b8 01 00 00 	add    $0x1b8,%rsp
    8238:	5b                   	pop    %rbx
    8239:	41 5c                	pop    %r12
    823b:	41 5d                	pop    %r13
    823d:	41 5e                	pop    %r14
    823f:	41 5f                	pop    %r15
    8241:	5d                   	pop    %rbp
    8242:	c3                   	retq   

0000000000009630 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E>:
    9630:	48 8b 47 20          	mov    0x20(%rdi),%rax
    9634:	4c 8b 4f 28          	mov    0x28(%rdi),%r9
    9638:	49 39 c1             	cmp    %rax,%r9
    963b:	0f 86 7e 00 00 00    	jbe    96bf <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E+0x8f>
    9641:	4c 8b 17             	mov    (%rdi),%r10
    9644:	48 8b 7f 10          	mov    0x10(%rdi),%rdi
    9648:	44 89 c9             	mov    %r9d,%ecx
    964b:	29 c1                	sub    %eax,%ecx
    964d:	4d 8d 41 ff          	lea    -0x1(%r9),%r8
    9651:	49 29 c0             	sub    %rax,%r8
    9654:	48 83 e1 03          	and    $0x3,%rcx
    9658:	74 1c                	je     9676 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E+0x46>
    965a:	48 f7 d9             	neg    %rcx
    965d:	0f 1f 00             	nopl   (%rax)
    9660:	48 63 14 87          	movslq (%rdi,%rax,4),%rdx
    9664:	49 0f af 14 c2       	imul   (%r10,%rax,8),%rdx
    9669:	48 8d 40 01          	lea    0x1(%rax),%rax
    966d:	48 01 d6             	add    %rdx,%rsi
    9670:	48 83 c1 01          	add    $0x1,%rcx
    9674:	75 ea                	jne    9660 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E+0x30>
    9676:	49 83 f8 03          	cmp    $0x3,%r8
    967a:	72 43                	jb     96bf <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E+0x8f>
    967c:	0f 1f 40 00          	nopl   0x0(%rax)
    9680:	48 63 0c 87          	movslq (%rdi,%rax,4),%rcx
    9684:	49 0f af 0c c2       	imul   (%r10,%rax,8),%rcx
    9689:	48 01 f1             	add    %rsi,%rcx
    968c:	48 63 54 87 04       	movslq 0x4(%rdi,%rax,4),%rdx
    9691:	49 0f af 54 c2 08    	imul   0x8(%r10,%rax,8),%rdx
    9697:	48 01 ca             	add    %rcx,%rdx
    969a:	48 63 4c 87 08       	movslq 0x8(%rdi,%rax,4),%rcx
    969f:	49 0f af 4c c2 10    	imul   0x10(%r10,%rax,8),%rcx
    96a5:	48 01 d1             	add    %rdx,%rcx
    96a8:	48 63 74 87 0c       	movslq 0xc(%rdi,%rax,4),%rsi
    96ad:	49 0f af 74 c2 18    	imul   0x18(%r10,%rax,8),%rsi
    96b3:	48 8d 40 04          	lea    0x4(%rax),%rax
    96b7:	48 01 ce             	add    %rcx,%rsi
    96ba:	49 39 c1             	cmp    %rax,%r9
    96bd:	75 c1                	jne    9680 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h0a25f40146e58c77E+0x50>
    96bf:	48 89 f0             	mov    %rsi,%rax
    96c2:	c3                   	retq   
    96c3:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
    96ca:	00 00 00 
    96cd:	0f 1f 00             	nopl   (%rax)

00000000000096d0 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE>:
    96d0:	48 8b 17             	mov    (%rdi),%rdx
    96d3:	44 8b 57 08          	mov    0x8(%rdi),%r10d
    96d7:	8b 47 20             	mov    0x20(%rdi),%eax
    96da:	83 c0 01             	add    $0x1,%eax
    96dd:	41 b8 ab aa aa aa    	mov    $0xaaaaaaab,%r8d
    96e3:	45 31 c9             	xor    %r9d,%r9d
    96e6:	41 8d 7a 01          	lea    0x1(%r10),%edi
    96ea:	48 85 d2             	test   %rdx,%rdx
    96ed:	75 3f                	jne    972e <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0x5e>
    96ef:	e9 ac 00 00 00       	jmpq   97a0 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd0>
    96f4:	66 66 66 2e 0f 1f 84 	data16 data16 nopw %cs:0x0(%rax,%rax,1)
    96fb:	00 00 00 00 00 
    9700:	89 f9                	mov    %edi,%ecx
    9702:	0f af c8             	imul   %eax,%ecx
    9705:	48 89 ca             	mov    %rcx,%rdx
    9708:	49 0f af d0          	imul   %r8,%rdx
    970c:	48 c1 ea 21          	shr    $0x21,%rdx
    9710:	8d 14 52             	lea    (%rdx,%rdx,2),%edx
    9713:	39 d1                	cmp    %edx,%ecx
    9715:	41 0f 45 c9          	cmovne %r9d,%ecx
    9719:	01 f1                	add    %esi,%ecx
    971b:	83 c0 01             	add    $0x1,%eax
    971e:	41 89 fa             	mov    %edi,%r10d
    9721:	31 d2                	xor    %edx,%edx
    9723:	89 ce                	mov    %ecx,%esi
    9725:	41 8d 7a 01          	lea    0x1(%r10),%edi
    9729:	48 85 d2             	test   %rdx,%rdx
    972c:	74 72                	je     97a0 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd0>
    972e:	83 ff 05             	cmp    $0x5,%edi
    9731:	0f 87 8c 00 00 00    	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9737:	48 85 d2             	test   %rdx,%rdx
    973a:	74 69                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    973c:	41 8d 7a 02          	lea    0x2(%r10),%edi
    9740:	83 ff 05             	cmp    $0x5,%edi
    9743:	0f 87 7a 00 00 00    	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9749:	48 83 fa 01          	cmp    $0x1,%rdx
    974d:	74 56                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    974f:	41 8d 7a 03          	lea    0x3(%r10),%edi
    9753:	83 ff 05             	cmp    $0x5,%edi
    9756:	77 6b                	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9758:	48 83 fa 02          	cmp    $0x2,%rdx
    975c:	74 47                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    975e:	41 8d 7a 04          	lea    0x4(%r10),%edi
    9762:	83 ff 05             	cmp    $0x5,%edi
    9765:	77 5c                	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9767:	48 83 fa 03          	cmp    $0x3,%rdx
    976b:	74 38                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    976d:	41 8d 7a 05          	lea    0x5(%r10),%edi
    9771:	83 ff 05             	cmp    $0x5,%edi
    9774:	77 4d                	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9776:	48 83 fa 04          	cmp    $0x4,%rdx
    977a:	74 29                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    977c:	41 83 fa fa          	cmp    $0xfffffffa,%r10d
    9780:	72 41                	jb     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    9782:	48 83 fa 05          	cmp    $0x5,%rdx
    9786:	75 28                	jne    97b0 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xe0>
    9788:	41 83 c2 06          	add    $0x6,%r10d
    978c:	44 89 d7             	mov    %r10d,%edi
    978f:	83 f8 05             	cmp    $0x5,%eax
    9792:	0f 86 68 ff ff ff    	jbe    9700 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0x30>
    9798:	eb 29                	jmp    97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    979a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
    97a0:	83 ff 05             	cmp    $0x5,%edi
    97a3:	77 1e                	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    97a5:	83 f8 05             	cmp    $0x5,%eax
    97a8:	0f 86 52 ff ff ff    	jbe    9700 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0x30>
    97ae:	eb 13                	jmp    97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    97b0:	41 83 c2 07          	add    $0x7,%r10d
    97b4:	41 83 fa 05          	cmp    $0x5,%r10d
    97b8:	77 09                	ja     97c3 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xf3>
    97ba:	44 89 d7             	mov    %r10d,%edi
    97bd:	48 83 fa 06          	cmp    $0x6,%rdx
    97c1:	74 e2                	je     97a5 <_ZN84_$LT$core..iter..Map$LT$I$C$$u20$F$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4fold17h3587eff9f2c27badE+0xd5>
    97c3:	89 f0                	mov    %esi,%eax
    97c5:	c3                   	retq   
    97c6:	66 2e 0f 1f 84 00 00 	nopw   %cs:0x0(%rax,%rax,1)
    97cd:	00 00 00 

