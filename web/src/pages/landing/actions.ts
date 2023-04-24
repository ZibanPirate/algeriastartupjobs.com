import { getState, getStateActions } from "src/state";
import Axios from "axios";
import { getConfig } from "src/utils/config/get-config";
import { JobPost, JobPostCategory } from "./state";
import { isLoaded } from "src/utils/loadable";

export const fetchJobsForFirstCategoryForLanding = async (): Promise<void> => {
  const { landingPage } = getStateActions();
  const { jobsPostsGroupedByCategory } = getState().landingPage;
  const [firstCategory, ...restOfJobsPostsGroupedByCategory] =
    jobsPostsGroupedByCategory;

  if (!firstCategory) return;
  if (isLoaded(firstCategory.job_posts)) return;

  try {
    // @TODO-ZM: use fetchV2
    // const { jobPosts } = await fetchV2("api:job-posts/dzcode", {});
    const { data: item } = await Axios.get<{
      category: JobPostCategory;
      job_posts: JobPost[];
    }>(
      getConfig().api.base_url +
        "/job-posts?category=" +
        firstCategory.category.name
    );

    landingPage.set({
      jobsPostsGroupedByCategory: [item, ...restOfJobsPostsGroupedByCategory],
    });
  } catch (error) {
    landingPage.set({
      jobsPostsGroupedByCategory: [
        { ...firstCategory, job_posts: "ERROR" },
        ...restOfJobsPostsGroupedByCategory,
      ],
    });
    // @TODO-ZM: add sentry
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
