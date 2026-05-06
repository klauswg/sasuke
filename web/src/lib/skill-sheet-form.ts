import type { SkillContentVm, SkillMetaVm } from '@/types';

export type SkillSheetMode = 'view' | 'create' | 'edit';

export interface SkillFormState {
  name: string;
  description: string;
  body: string;
  source: string;
}

export interface SkillSaveRequest {
  name: string;
  scope: string;
  wsPath: string | null;
  content: string;
  oldName: string | null;
  directoryPath: string | null;
  syncTargets: string[];
}

export function createEmptySkillForm(source: string): SkillFormState {
  return { name: '', description: '', body: '', source };
}

export function createSkillFormFromContent(
  content: SkillContentVm | null | undefined,
  fallbackSource: string,
): SkillFormState {
  return {
    name: content?.meta.name ?? '',
    description: content?.descriptionSource ?? content?.meta.description ?? '',
    body: content?.body ?? '',
    source: (content?.meta.source as string | undefined) ?? fallbackSource,
  };
}

export function filterSkillSyncTargets(
  current: string[],
  availableAgents: Array<{ agentType: string }>,
) {
  const available = new Set(availableAgents.map((agent) => agent.agentType));
  return current.filter((agentType) => available.has(agentType));
}

export function buildSkillSaveRequest(input: {
  form: SkillFormState;
  mode: Exclude<SkillSheetMode, 'view'>;
  editTarget: SkillMetaVm | null;
  editWorkspacePath: string | null;
  syncTargets: string[];
}): SkillSaveRequest {
  const scope = input.form.source.startsWith('project:') ? 'project' : input.form.source;
  const wsPath = input.mode === 'edit'
    ? input.editWorkspacePath
    : (input.form.source.startsWith('project:') ? input.form.source.slice(8) : null);
  const name = input.form.name.trim();
  return {
    name,
    scope,
    wsPath,
    content: `---\nname: ${frontmatterScalar(name)}\ndescription: ${frontmatterScalar(input.form.description.trim())}\n---\n\n${input.form.body}`,
    oldName: input.mode === 'edit' ? input.editTarget?.name ?? null : null,
    directoryPath: input.mode === 'edit' ? input.editTarget?.directoryPath ?? null : null,
    syncTargets: input.syncTargets,
  };
}

function frontmatterScalar(value: string): string {
  if (value.includes('\n')) {
    const lines = value.split(/\r?\n/);
    return `|\n${lines.map((line) => `  ${line}`).join('\n')}`;
  }
  if (/^[A-Za-z0-9_.-]+$/.test(value)) return value;
  return JSON.stringify(value);
}


/** SKILL.md 正文中的相对引用解析：跳过 http(s)/锚点/协议相对/绝对路径，返回 skill 目录内相对路径（允许 ../ 指向兄弟 skill 目录，由后端在 skills 根内校验）。 */
export function resolveSkillRelativeHref(href: string): string | null {
  const raw = href.trim();
  if (!raw || /^(?:[a-z][a-z0-9+.-]*:|#|\/\/)/i.test(raw)) return null;
  const path = raw.split('#')[0].split('?')[0];
  if (!path) return null;
  try {
    return decodeURIComponent(path);
  } catch {
    return path;
  }
}

/** 以当前打开文件为基准解析相对引用（保留必要的 ../ 段，后端在 skills 根内校验）。 */
export function resolveSkillPathFrom(currentPath: string | null, href: string): string {
  const base = currentPath ? currentPath.split('/').slice(0, -1).join('/') : '';
  const raw = base ? `${base}/${href}` : href;
  const segments: string[] = [];
  for (const segment of raw.split('/')) {
    if (!segment || segment === '.') continue;
    if (segment === '..') {
      if (segments.length > 0 && segments[segments.length - 1] !== '..') segments.pop();
      else segments.push('..');
      continue;
    }
    segments.push(segment);
  }
  return segments.join('/');
}
