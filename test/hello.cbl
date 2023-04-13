       identification division.
       program-id. hello.
       environment division.
       DATA division.
       Working-storage section.
       01 ab PIC xx value "ab".
       01 cd pic xx value "cd".
       procedure division.
       DisPlay cd.
       move ab to cd.
       accept ab.
       DisPlay cd.
       DisPlay ab.
