import {message} from 'antd';

export function handleError(error: unknown): void {
  if (!error || typeof error !== "object" || !("response" in error)) {
    return;
  }
  const response = (error as { response?: unknown }).response;
  if (!response || typeof response !== "object" || !("data" in response)) {
    return;
  }
  const data = (response as { data: unknown }).data as Record<string, unknown>;
  if(data.code!=0) {
    if (data.data
        &&typeof data.data === "object"
        && "validationErrors" in data.data
        && Array.isArray((data.data as Record<string, unknown>).validationErrors)
        && (data.data as Record<string, unknown[]>).validationErrors.length > 0 ) { //服务端验证错误（validationErrors）
      let validationErrorMessage = "";
      const validationErrors = (data.data as Record<string, unknown[]>).validationErrors;
      for (let i = 0; i < validationErrors.length; i++) {
        const validationError = validationErrors[i];
        if (validationError
            && typeof validationError === "object"
            && "message" in validationError) {
          const msg = (validationError as Record<string, unknown>).message;
          if (typeof msg === "string") {
            validationErrorMessage += msg + "；\n";
          }
        }
      }
      if (validationErrorMessage) {
        message?.error(validationErrorMessage);
      }
    } else if ("msg" in data) { //服务端自定义错误消息（message）
      const msg = data.msg;
      if (typeof msg === "string") {
        message?.error(msg);
      }
    } else if ("data" in data) { //服务端未成功响应但提供了详细错误（details）
      const msg = data.data;
      if (typeof msg === "string") {
        message?.error(msg);
      }
    }
  } else if("statusText" in response){
    const statusText = response.statusText;
    if (typeof statusText === "string") {
      message?.error(statusText);
    }
  }
}
