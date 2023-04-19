import { getStateActions } from "src/state";

export const fetchJobPostsForLanding = async (): Promise<void> => {
  const { landingPage } = getStateActions();
  try {
    landingPage.set({ jobPosts: null });
    // @TODO-ZM: use fetchV2
    // const { milestones } = await fetchV2("api:MileStones/dzcode", {});
    await new Promise((resolve) => setTimeout(resolve, 1000));

    landingPage.set({
      jobPosts: [
        {
          id: "1",
          title: "Frontend Developer",
          description:
            "We are looking for a frontend developer to join our team",
        },
        {
          id: "2",
          title: "Backend Developer",
          description:
            "We are looking for a backend developer to join our team",
        },
        {
          id: "3",
          title: "Fullstack Developer",
          description:
            "We are looking for a fullstack developer to join our team",
        },
      ],
    });
  } catch (error) {
    landingPage.set({ jobPosts: "ERROR" });
    // @TODO-ZM: add sentry
    // Sentry.captureException(error, { tags: { type: "WEB_FETCH" } });
  }
};
