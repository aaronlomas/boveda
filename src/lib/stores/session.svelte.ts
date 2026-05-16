export class SessionState {
  isUnlocked = $state(false);
}

export const sessionState = new SessionState();
