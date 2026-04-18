import AppConsts from "./app-consts";
import axios, { AxiosInstance } from "axios";
import {message} from 'antd';
import { handleError } from "./error-handler";
import {storageUtils} from "@/utils/storageUtils.ts";

export interface IResponse {
  code: number | string;
  data: any;
  msg: string;
  total: number
}

let _instance: AxiosInstance | null = null;

export function apiHttp(): AxiosInstance {
  if (!_instance) {
    _instance = axios.create({
      baseURL: AppConsts.remoteServiceBaseUrl,
    });
    _instance.defaults.withCredentials = true;

    _instance.interceptors.response.use(
      (r) => {
        // const d = r?.data as Record<string, unknown>;
        // if ( d && typeof d === "object" && d.__abp && "result" in d &&(d.success === undefined || d.success === true)) {
        //   r.data = d.result;
        // }
        return r;
      },
      (error) => {
        handleError(error);
        return Promise.reject(error);
      },
    );

    _instance.interceptors.request.use((cfg) => {
      try {
        cfg.headers = cfg.headers || ({} as Record<string, unknown>);
        const token = storageUtils.getToken();
        if (token && !cfg.headers["Authorization"]) {
          cfg.headers["Authorization"] = `Bearer ${token}`;
        }
        try {
          const m = document.cookie.match(
            /(?:^|; )Abp\.Localization\.CultureName=([^;]*)/,
          );
          const cookieLang = m ? decodeURIComponent(m[1]) : "";
          if (cookieLang && !cfg.headers[".AspNetCore.Culture"]) {
            cfg.headers[".AspNetCore.Culture"] =
              `c=${cookieLang}|uic=${cookieLang}`;
          }
        } catch {
          /* ignore */
        }
        try {
          const tenantCookieName = "X-TenantId";
          const getTenantId = () => {
            try {
              return undefined;
            } catch {
              return undefined;
            }
          };
          let tenantId: string | number | undefined = getTenantId();
          if (!tenantId) {
            const re = new RegExp(
              "(?:^|; )" +
                tenantCookieName.replace(/([.$?*|{}()[\\/+^])/g, "\\$1") +
                "=([^;]*)",
            );
            const m2 = document.cookie.match(re);
            tenantId = m2 ? decodeURIComponent(m2[1]) : "";
          }
          if (
            tenantId &&
            !(tenantCookieName in (cfg.headers as Record<string, unknown>))
          ) {
            (cfg.headers as Record<string, unknown>)[tenantCookieName] =
              tenantId + "";
          }
        } catch {
          message?.error?.("An error occurred, please try again later.");
        }
        if (!cfg.headers["Accept"]) cfg.headers["Accept"] = "application/json";
      } catch {
        message?.error?.("An error occurred, please try again later.");
      }
      return cfg;
    });
  }
  return _instance;
}

export default apiHttp;
