/**

basic supported grammar

query -> select (into)?

select -> SELECT select-list FROM table-expr

          (
into ->

query:
    SELECT <columns>	5.
    FROM <table>	1.
    WHERE <predicate on rows>	2.
    GROUP BY <columns>	3.
    ORDER BY <columns>	4.
    INTO <target> 6.

columns:
    (column,?)+
**/