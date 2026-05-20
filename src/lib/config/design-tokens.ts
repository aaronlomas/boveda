/**
 * Design Token System
 * Define colores, espaciados, bordes y clases compuestas estandarizadas.
 * Úsalo en todos los componentes para mantener la consistencia visual.
 */

export const COLORS = {
  surface: {
    DEFAULT: "bg-surface",
    subtle: "bg-surface/3",
    3: "bg-surface/3",
    4: "bg-surface/4",
    5: "bg-surface/5",
    7: "bg-surface/7",
    8: "bg-surface/8",
    10: "bg-surface/10",
    15: "bg-surface/15",
    20: "bg-surface/20",
    border: "border-surface/8",
    borderLight: "border-surface/10",
    borderMedium: "border-surface/20",
    borderHover: "border-surface/30",
  },
  accent: {
    DEFAULT: "bg-accent",
    text: "text-accent",
    light: "text-accent-light bg-accent/10",
    hover: "hover:bg-accent-light",
    border: "border-accent",
    borderLight: "border-accent/20",
    borderHover: "hover:border-accent/40",
  },
  text: {
    primary: "text-text-primary",
    secondary: "text-text-secondary",
    muted: "text-text-muted",
    danger: "text-danger",
    warning: "text-warning",
    success: "text-success",
  },
  status: {
    danger: "bg-danger/10 text-danger border-danger/20",
    warning: "bg-warning/10 text-warning border-warning/20",
    success: "bg-success/10 text-success border-success/20",
  }
} as const;

export const SIZING = {
  button: {
    sm: "px-3 py-1.5 text-xs",
    md: "px-4 py-2 text-sm",
    lg: "px-6 py-2.5 text-base",
    icon: "p-2",
  },
  card: "p-5 rounded-2xl",
  panel: "p-5 rounded-xl",
  input: "px-3 py-2 rounded-md",
  modal: "p-6 rounded-2xl",
  border: "border",
  shadow: {
    sm: "shadow-sm",
    md: "shadow-md",
    lg: "shadow-lg",
    xl: "shadow-xl",
    accent: "shadow-accent/20",
  }
} as const;

export const ANIMATION = {
  transitionAll: "transition-all duration-300",
  transitionColors: "transition-colors duration-200",
  hoverScale: "hover:scale-105 active:scale-95",
  hoverLift: "hover:-translate-y-0.5",
  fadeIn: "animate-in fade-in duration-300",
  slideUp: "animate-in fade-in slide-in-from-bottom-4 duration-500",
} as const;

export const COMPONENTS = {
  card: `${COLORS.surface[4]} ${SIZING.border} ${COLORS.surface.border} ${SIZING.card} ${SIZING.shadow.xl} ${ANIMATION.transitionAll} hover:${COLORS.surface.borderHover}`,
  panel: `${COLORS.surface[3]} ${SIZING.border} ${COLORS.surface.border} ${SIZING.panel}`,
  buttonPrimary: `${COLORS.accent.DEFAULT} text-white font-bold ${SIZING.button.md} rounded-xl ${ANIMATION.transitionAll} ${ANIMATION.hoverLift} ${SIZING.shadow.lg} shadow-accent/20`,
  buttonSecondary: `${COLORS.surface[10]} ${COLORS.text.primary} font-medium ${SIZING.button.md} rounded-xl ${ANIMATION.transitionAll} hover:${COLORS.surface[20]}`,
  buttonGhost: `bg-transparent ${COLORS.text.secondary} font-medium ${SIZING.button.md} rounded-xl ${ANIMATION.transitionAll} hover:${COLORS.surface[10]} hover:${COLORS.text.primary}`,
  input: `flex w-full ${SIZING.input} bg-surface/4 ${SIZING.border} ${COLORS.surface.borderLight} text-sm ${COLORS.text.primary} outline-none focus:${COLORS.accent.border} focus:ring-2 focus:ring-accent/50 ${ANIMATION.transitionAll}`,
} as const;
