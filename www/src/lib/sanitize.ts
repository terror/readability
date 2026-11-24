import DOMPurify from 'dompurify';

DOMPurify.addHook('afterSanitizeAttributes', (node) => {
  if (node.tagName === 'A') {
    node.setAttribute('target', '_blank');
    node.setAttribute('rel', 'noopener noreferrer');
  }
});

export const sanitize = (html: string) =>
  DOMPurify.sanitize(html, {
    FORBID_ATTR: ['style'],
    FORBID_TAGS: ['button', 'form', 'input', 'style'],
  });
