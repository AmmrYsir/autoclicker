/**
 * Reusable Tailwind class constants for consistent styling
 */

// Container & Layout
export const CONTAINER = {
	section: 'mb-6 p-4 bg-slate-700/30 rounded-lg border border-slate-600/50',
	header: 'flex items-center justify-between mb-6',
	flexCol: 'flex flex-col',
	flexRow: 'flex items-center',
};

// Typography
export const TEXT = {
	label: 'text-xs font-semibold text-slate-400 uppercase tracking-wider',
	labelSmall: 'text-xs text-slate-500 uppercase tracking-wider',
	mono: 'text-xs font-mono',
};

// Buttons
export const BUTTON = {
	base: 'transition-all duration-200 flex items-center justify-center font-medium',
	small: 'w-10 h-10 bg-slate-700 hover:bg-slate-600 text-white rounded-lg',
	iconSmall: 'w-5 h-5',
};

// Input
export const INPUT = {
	number: 'w-full bg-slate-900/50 text-white text-center text-2xl font-bold py-2 rounded-lg border border-slate-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent',
};

// Badge & Status
export const BADGE = {
	status: 'w-2 h-2 rounded-full',
	hotkey: 'px-2 py-1 bg-slate-700 text-slate-300 text-xs font-mono rounded',
};

// State colors
export const COLORS = {
	running: 'bg-orange-400 animate-pulse',
	ready: 'bg-emerald-400',
};
