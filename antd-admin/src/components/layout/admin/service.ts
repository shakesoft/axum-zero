/**
 * @description: 菜单列表
 * @params {param} ListParam
 * @return {Promise}
 */
import apiHttp, {IResponse} from "@/api/http-client.ts";

export const query_user_menu = (): Promise<IResponse> => {
    return apiHttp().get('api/system/user/queryUserMenu', {}).then(res => res.data);
};

