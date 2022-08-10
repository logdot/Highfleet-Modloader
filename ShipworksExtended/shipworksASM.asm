.data
; External functions
New             QWORD 140309078h
CreateLibraryEx QWORD 14025cf50h
MakeGlobalEx    QWORD 140187120h
CopyStringEx    QWORD 1400051c0h
DefinePartEx    QWORD 1401FDE40h

EditorParts     QWORD 143942568h ; Object to the object that holds all editor parts

; LIBRARIES
EditorLibraryHull           QWORD 14038d420h
EditorLibraryArmor          QWORD 14038d438h
EditorLibraryFuel           QWORD 14038d450h
EditorLibraryAmmo           QWORD 14038d468h
EditorLibraryLeg            QWORD 14038d480h
EditorLibraryEngines        QWORD 14038d498h
EditorLibraryGuns           QWORD 14038d4b0h
EditorLibrarySensors        QWORD 14038d4c8h
EditorLibraryExpendables    QWORD 14038d4e0h
EditorLibraryCrafts         QWORD 14038d4f8h
EditorLibraryDefense        QWORD 14038d510h
EditorLibraryGenerators     QWORD 14038d528h
EditorLibraryEmergency      QWORD 14038d558h

mdlEngine BYTE "MDL_ENGINE_10", 00

.code

public DefinePart
DefinePart PROC
    sub rsp, 72 ; Need to assure that the stack is 32 bit's aligned otherwise a movaps crashes the game.
                ; Additionally need enough stack space for DefinePartEx to encroach on
    mov r9d, 1
    ;mov r8, [EditorLibraryEngines]
    mov r8, [r8]
    ;lea rdx, [mdlEngine]
    ;mov rcx, [EditorParts]
    mov rcx, [rcx]
    call DefinePartEx
    mov dword ptr [rax+798h], 3E7h
    add rsp, 72
    ret
DefinePart ENDP 

; __fast_call
; void CreateLibrary(Library library, char* libraryName)
; RDX = libraryName
; RCX = library
public CreateLibrary
CreateLibrary PROC
    push rbx
    mov rbx, [rcx]
    test rbx, rbx
    jz noDeconstruct
    mov rax, [rbx]
    call qword ptr [rax+48h]
noDeconstruct:
    ;; Save our parameters
    push rdx
    push rcx
    mov rcx, 128h
    call New
    test rax, rax
    jz newFail
    mov rcx, rax
    sub rsp, 40
    call CreateLibraryEx
    add rsp, 40
    jmp newSuccess
newFail:
    xor rbx, rbx
    mov rax, rbx
newSuccess:
    mov rdx, rax
    pop rbx ; Load our first parameter into rbx
    lea rcx, [rbx-8]
    sub rsp, 40
    call MakeGlobalEx
    add rsp, 40
    mov rcx, [rbx]
    add rcx, 0c8h
    mov r8d, 13h
    pop rdx
    sub rsp, 40
    call CopyStringEx
    add rsp, 40
    pop rbx
    ret
CreateLibrary ENDP
END