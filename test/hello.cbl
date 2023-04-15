       identification division.
       program-id. hello.
       environment division.
       DATA division.
       Working-storage section.
       01 ab PIC xx value "ab".
       01 cd pic xx value "cd".
       procedure division.
       move ab to cd.
       DisPlay cd.
       go to end-label.
       DisPlay ab.
