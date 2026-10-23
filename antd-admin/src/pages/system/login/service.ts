import {ILogin} from "./data";
import {apiHttp, IResponse} from "@/api/http-client.ts";

/**
 * @description: 用户登录
 * @params {ILogin} params
 * @return {Promise}
 */
export const reqLogin = (params: ILogin): Promise<IResponse> => {
    return apiHttp().post('api/system/user/login', params).then(res => res.data);
};

/**
 * @description: 退出登录（删除服务端会话）
 */
export const reqLogout = (refreshToken: string): Promise<IResponse> => {
    return apiHttp().post('api/system/user/logout', {refreshToken}).then(res => res.data);
};
