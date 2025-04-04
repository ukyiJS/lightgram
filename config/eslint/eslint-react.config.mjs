import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';

export const eslintReactConfig = [
  {
    files: ['**/*.{jsx,tsx}'],
    plugins: {
      react,
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
    },
    rules: {
      /* 리액트 훅 관련 권장 규칙 */
      ...reactHooks.configs.recommended.rules,

      /* react-refresh: HMR 안정성을 위해 컴포넌트만 export 하도록 제한 */
      'react-refresh/only-export-components': ['warn', {
        allowConstantExport: true,
      }],

      /* boolean props는 값 없이 작성 (e.g. <input disabled />) */
      'react/jsx-boolean-value': ['error', 'never'],

      /* JSX 닫는 괄호 위치를 일관되게 설정 */
      'react/jsx-closing-bracket-location': 'error',

      /* 닫는 태그의 위치를 일관되게 설정 */
      'react/jsx-closing-tag-location': 'error',

      /* props와 children에서 불필요한 중괄호 제거 (e.g. title={"hi"} → title="hi") */
      'react/jsx-curly-brace-presence': ['error', {
        props: 'never',
        children: 'never',
      }],

      /* 중괄호 안 공백 금지 (children은 허용) */
      'react/jsx-curly-spacing': ['error', {
        when: 'never',
        attributes: {
          allowMultiline: false,
        },
        children: true,
      }],

      /* = 앞뒤에 공백 없이 작성 (e.g. prop="value") */
      'react/jsx-equals-spacing': ['error', 'never'],

      /* JSX에서 줄바꿈 방지 설정 */
      'react/jsx-newline': ['error', {
        prevent: true,
      }],

      /* 쓸모없는 Fragment 제거 (e.g. <></>만 사용하는 경우) */
      'react/jsx-no-useless-fragment': 'error',

      /* JSX prop 사이에 여러 공백 금지 */
      'react/jsx-props-no-multi-spaces': 'error',

      /* JSX prop 정렬: reserved → 일반 → callback → shorthand 순서 */
      'react/jsx-sort-props': ['error', {
        noSortAlphabetically: false,
        reservedFirst: true,
        shorthandLast: true,
        callbacksLast: true,
      }],

      /* 태그 간격 설정 (e.g. <Component /> 형태로) */
      'react/jsx-tag-spacing': ['error', {
        closingSlash: 'never',
        beforeSelfClosing: 'always',
        beforeClosing: 'never',
      }],

      /* 셀프 클로징 가능한 태그는 자동 닫힘 적용 (e.g. <img />) */
      'react/self-closing-comp': ['error', {
        component: true,
        html: true,
      }],

      /* 이벤트 핸들러는 handleXXX, props는 onXXX 네이밍 규칙 적용 */
      'react/jsx-handler-names': ['error', {
        eventHandlerPrefix: 'handle',
        eventHandlerPropPrefix: 'on',
      }],
    },
  },
];
