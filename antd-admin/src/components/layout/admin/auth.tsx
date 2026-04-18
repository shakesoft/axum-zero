import {storageUtils} from "@/utils/storageUtils.ts";
import {Navigate} from "react-router-dom"
import {ReactNode} from "react";

function isAuthenticated() {
  const token = storageUtils.getToken();
  const expiresAt = storageUtils.getExpiresAt();
  if (!token || !expiresAt) return false;
  return Math.floor(Date.now() / 1000) < Number(expiresAt);
}

interface AuthGuardProps {
  children: ReactNode;
}

export default function AuthGuard({ children }: AuthGuardProps) {
  if (!isAuthenticated()) {
    return <Navigate to="/login" replace />;
  }
  return <>{children}</>;
}
