import { FC, useEffect, useRef, useState } from "react";
import "./style.css";
import "src/utils/css/index.css";
import { Route } from "react-router-dom";
import { LazyPages, pageLoaders } from "src/pages";
import { useHtmlThemeColor } from "src/utils/hooks/html-theme-color";
import {
  CONFIRM_EMAIL_PAGE_URL,
  CONFIRM_LOGIN_PAGE_URL,
  CREATE_POST_PAGE_URL,
  LOGIN_PAGE_URL,
  ME_PAGE_URL,
  POST_PAGE_URL,
} from "src/utils/urls/common";
import LoadingBar, { LoadingBarRef } from "react-top-loading-bar";
import { viewTransition } from "src/utils/animation/view-transition";
import { SentryRoutes } from "src/components/router-provider";

export const App: FC = () => {
  useHtmlThemeColor();

  // @TODO-ZM: refactor this into a DeferredRoutes component
  const loadingBarRef = useRef<LoadingBarRef>(null);
  const [pageToRender, setPageToRender] = useState("");
  const [currentPage, setCurrentPage] = useState("");
  const { current: loadedPages } = useRef<string[]>([]);

  const pageToRenderSetter =
    (page: string): FC =>
    () => {
      useEffect(() => setPageToRender(page), []);
      return null;
    };

  const asyncUseEffect = async () => {
    if (currentPage === pageToRender) return;
    if (!currentPage && pageToRender) setCurrentPage(pageToRender);

    try {
      if (!loadedPages.includes(pageToRender)) {
        loadingBarRef.current?.continuousStart();
        await pageLoaders[pageToRender]?.();
        loadingBarRef.current?.complete();
        loadedPages.push(pageToRender);
      }
    } catch (error) {}

    viewTransition(async () => {
      setCurrentPage(pageToRender);
      await new Promise((resolve) => setTimeout(resolve, 10));
    });
  };

  useEffect(() => {
    asyncUseEffect();
  }, [currentPage, pageToRender, loadingBarRef.current]);

  return (
    <>
      <LoadingBar color="#41aa55" ref={loadingBarRef} height={4} />
      <SentryRoutes>
        <Route path="/" Component={pageToRenderSetter("landing")} />
        <Route path={POST_PAGE_URL} Component={pageToRenderSetter("post")} />
        <Route path={CREATE_POST_PAGE_URL} Component={pageToRenderSetter("create-post")} />
        <Route path={CONFIRM_EMAIL_PAGE_URL} Component={pageToRenderSetter("confirm-email")} />
        <Route path={LOGIN_PAGE_URL} Component={pageToRenderSetter("login")} />
        <Route path={CONFIRM_LOGIN_PAGE_URL} Component={pageToRenderSetter("confirm-login")} />
        <Route path={ME_PAGE_URL} Component={pageToRenderSetter("me")} />
        <Route path="*" Component={pageToRenderSetter("404")} />
      </SentryRoutes>
      {LazyPages[currentPage]}
    </>
  );
};
