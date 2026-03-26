grammar Alice;

/* Parser rules */
aliceScript: statements EOF;

statements: (statement SEMIS)+;
statement: usingNamespace;

usingNamespace: USING NAMESPACE identifierPath;

identifierPath: '::'? identifier ('::' identifier)*;
identifier: KEYWORD | IDENTIFIER;

/* Lexer rules */
SEMIS: ';' | NL (';' | NL)*;

NL: LF | (CR LF?);

BLOCK_COMMENT: '/*' ( BLOCK_COMMENT | ~[*/])* '*/';
LINE_COMMENT: '//' ~[NL]*;
WS: ' ' | '\t' | '\f';
SKIPPED: (BLOCK_COMMENT | LINE_COMMENT | WS) -> skip;

IDENTIFIER: '_' | [a-zA-Z][a-zA-Z0-9_]*;

/* Keywords */
ID: 'id';
PROP: 'prop';
SYS: 'sys';
QUERY: 'query';
INCLUDE: 'include';
NAMESPACE: 'namespace';
CONST: 'const';
AND: 'and';
OR: 'or';
USING: 'using';
NOT: 'not';

KEYWORD:
	ID
	| PROP
	| SYS
	| QUERY
	| INCLUDE
	| NAMESPACE
	| CONST
	| AND
	| OR
	| USING
	| NOT;

fragment LF: '\n';
fragment CR: '\r';