/**
 * ESLint custom rule: param-object-newline
 * 조건:
 * - ObjectPattern, ArrayPattern, ObjectExpression, ImportDeclaration, ExportNamedDeclaration
 * - 항목 개수가 N개 이상일 경우 줄바꿈 + 들여쓰기 + 닫는 괄호 위치 정리
 */

const objectNewlinePlugin = {
  rules: {
    'object-newline': {
      meta: {
        type: 'layout',
        docs: {
          description: 'N개 이상의 구조분해 및 객체, import/export/array는 각 줄마다 하나씩 줄바꿈',
        },
        fixable: 'whitespace',
        schema: [
          {
            type: 'object',
            properties: {
              minProperties: {
                type: 'number',
                minimum: 1,
              },
              indent: {
                type: 'number',
                minimum: 1,
              },
            },
            additionalProperties: false,
          },
        ],
      },
      create(context) {
        const sourceCode = context.getSourceCode();
        const minProps = context.options?.[0]?.minProperties ?? 4;
        const indentSize = context.options?.[0]?.indent ?? 2;

        function getIndentLevel(token) {
          const line = sourceCode.getText(token);
          const match = line.match(/^(\s*)/);
          return match?.[1] ?? '';
        }

        function formatItems(items, baseIndent) {
          const INDENT = baseIndent + ' '.repeat(indentSize);
          return items
            .map(item => `${INDENT}${sourceCode.getText(item)},`)
            .join('\n');
        }

        function replaceInsideBraces(fixer, openToken, closeToken, items) {
          const baseIndent = getIndentLevel(openToken);
          const text = `{
${formatItems(items, baseIndent)}
${baseIndent}}`;
          return fixer.replaceTextRange([openToken.range[0], closeToken.range[1]], text);
        }

        return {
          FunctionDeclaration(node) {
            node.params.forEach((param) => {
              const open = sourceCode.getFirstToken(param);
              const close = sourceCode.getLastToken(param);
              const isMultiLine = param.loc.start.line !== param.loc.end.line;
              const isInline = param.properties.some(p => p.loc.start.line === open.loc.start.line);
              const allSeparate = param.properties.every((p, i, arr) => i === 0 || p.loc.start.line !== arr[i - 1].loc.start.line);

              if (
                param.type === 'ObjectPattern' &&
                param.properties.length >= minProps &&
                (isInline || isMultiLine) &&
                !allSeparate
              ) {
                context.report({
                  node: param,
                  message: `${minProps}개 이상의 파라미터는 각 줄마다 줄바꿈 해야 합니다`,
                  fix(fixer) {
                    return replaceInsideBraces(fixer, open, close, param.properties);
                  },
                });
              }
            });
          },

          VariableDeclarator(node) {
            const id = node.id;
            if (id.type === 'ObjectPattern') {
              const open = sourceCode.getFirstToken(id);
              const close = sourceCode.getLastToken(id);
              const isMultiLine = id.loc.start.line !== id.loc.end.line;
              const isInline = id.properties.some(p => p.loc.start.line === open.loc.start.line);
              const allSeparate = id.properties.every((p, i, arr) => i === 0 || p.loc.start.line !== arr[i - 1].loc.start.line);

              if (
                id.properties.length >= minProps &&
                (isMultiLine || isInline) &&
                !allSeparate
              ) {
                context.report({
                  node: id,
                  message: `${minProps}개 이상의 구조분해는 각 줄마다 줄바꿈 해야 합니다`,
                  fix(fixer) {
                    return replaceInsideBraces(fixer, open, close, id.properties);
                  },
                });
              }
            }

            if (id.type === 'ArrayPattern') {
              const open = sourceCode.getFirstToken(id);
              const close = sourceCode.getLastToken(id);
              if (
                id.elements.length >= minProps &&
                open.loc.start.line === close.loc.end.line
              ) {
                const elements = id.elements.filter(Boolean);
                context.report({
                  node: id,
                  message: `${minProps}개 이상의 배열 구조분해는 각 줄마다 줄바꿈 해야 합니다`,
                  fix(fixer) {
                    const baseIndent = getIndentLevel(open);
                    const INDENT = baseIndent + ' '.repeat(indentSize);
                    return fixer.replaceTextRange(
                      [open.range[0], close.range[1]],
                      `[
${elements.map(el => `${INDENT}${sourceCode.getText(el)},`).join('\n')}
${baseIndent}]`
                    );
                  },
                });
              }
            }
          },

          ObjectExpression(node) {
            const open = sourceCode.getFirstToken(node);
            const close = sourceCode.getLastToken(node);
            const isMultiLine = node.loc.start.line !== node.loc.end.line;
            const isInline = node.properties.some(p => p.loc.start.line === open.loc.start.line);
            const allPropsSeparateLines = node.properties.every((p, i, arr) => i === 0 || p.loc.start.line !== arr[i - 1].loc.start.line);

            if (
              node.properties.length >= minProps &&
              (isMultiLine || isInline) &&
              !allPropsSeparateLines
            ) {
              context.report({
                node,
                message: `${minProps}개 이상의 객체 속성이 한 줄 또는 인라인에 몰려있으면 줄바꿈 해야 합니다`,
                fix(fixer) {
                  return replaceInsideBraces(fixer, open, close, node.properties);
                },
              });
            }
          },

          ImportDeclaration(node) {
            const namedSpecifiers = node.specifiers.filter(s => s.type === 'ImportSpecifier');
            if (namedSpecifiers.length === 0) return;

            const open = sourceCode.getTokenBefore(namedSpecifiers[0], { filter: t => t.value === '{' });
            const close = sourceCode.getTokenAfter(namedSpecifiers[namedSpecifiers.length - 1], { filter: t => t.value === '}' });
            if (!open || !close) return;

            const isInline = namedSpecifiers.some(s => s.loc.start.line === open.loc.start.line);
            const allSeparate = namedSpecifiers.every((s, i, arr) => i === 0 || s.loc.start.line !== arr[i - 1].loc.start.line);

            if (
              namedSpecifiers.length >= minProps &&
              (isInline || node.loc.start.line !== node.loc.end.line) &&
              !allSeparate
            ) {
              context.report({
                node,
                message: `${minProps}개 이상의 import 항목은 각 줄마다 줄바꿈 되어야 합니다`,
                fix(fixer) {
                  const baseIndent = getIndentLevel(open);
                  return fixer.replaceTextRange(
                    [open.range[0], close.range[1]],
                    `{
${formatItems(namedSpecifiers, baseIndent)}
${baseIndent}}`
                  );
                },
              });
            }
          },

          ExportNamedDeclaration(node) {
            const namedSpecifiers = node.specifiers;
            if (namedSpecifiers.length === 0) return;

            const open = sourceCode.getTokenBefore(namedSpecifiers[0], { filter: t => t.value === '{' });
            const close = sourceCode.getTokenAfter(namedSpecifiers[namedSpecifiers.length - 1], { filter: t => t.value === '}' });
            if (!open || !close) return;

            const isInline = namedSpecifiers.some(s => s.loc.start.line === open.loc.start.line);
            const allSeparate = namedSpecifiers.every((s, i, arr) => i === 0 || s.loc.start.line !== arr[i - 1].loc.start.line);

            if (
              namedSpecifiers.length >= minProps &&
              (isInline || node.loc.start.line !== node.loc.end.line) &&
              !allSeparate
            ) {
              context.report({
                node,
                message: `${minProps}개 이상의 export 항목은 각 줄마다 줄바꿈 되어야 합니다`,
                fix(fixer) {
                  const baseIndent = getIndentLevel(open);
                  return fixer.replaceTextRange(
                    [open.range[0], close.range[1]],
                    `{
${formatItems(namedSpecifiers, baseIndent)}
${baseIndent}}`
                  );
                },
              });
            }
          },
        };
      },
    },
  },
}

export const objectNewline = {
  configs: {
    recommended: {
      files: ['**/*.{js,mjs,cjs,ts,cts,mts,jsx,tsx}'],
      plugins: {
        'object-newline': objectNewlinePlugin,
      },
      rules: {
        'object-newline/object-newline': ['error', { minProperties: 4 }],
      },
    },
  }
}