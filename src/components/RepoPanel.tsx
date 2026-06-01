import IconBookmark from "~icons/tabler/bookmark";
import IconGitCompare from "~icons/tabler/git-compare";

import SidebarSection from "@/components/SidebarSection";

export default function RepoPanel() {
  return (
    <div className="flex flex-col gap-0">
      <SidebarSection
        emptyLabel="No bookmarks"
        icon={IconBookmark}
        iconTone="bookmark"
        title="Bookmarks"
      />
      <SidebarSection
        emptyLabel="No queries"
        icon={IconGitCompare}
        iconTone="query"
        title="Queries"
      />
    </div>
  );
}
