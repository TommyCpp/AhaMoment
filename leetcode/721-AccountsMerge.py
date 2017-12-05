class Solution:
    def accountsMerge(self, accounts):
        """
        :type accounts: List[List[str]]
        :rtype: List[List[str]]
        """
        name_email = {}
        for account in accounts:
            if account[0] not in name_email.keys():
                name_email[account[0]] = [set(account[1:])]
            else:
                current_email = set(account[1:]) # the emails that current account have
                in_common = [] # how many email set has email in common with current_email
                for email_set in name_email[account[0]]:
                    intersection = current_email.intersection(email_set) 
                    if len(intersection) != 0:
                        in_common.append(email_set) # if has email in common with current_email
                
                if len(in_common) == 0:
                    name_email[account[0]].append(current_email) # if no email set has email in common with current_email
                else:
                    new_set = set(current_email) # the merged email set
                    for c in in_common: 
                        new_set = new_set.union(c)
                        name_email[account[0]].remove(c)  # delete the original email set 
                    name_email[account[0]].append(new_set) # add the new email set

        results = []
        for name, emails in name_email.items():
            for email in emails:
                sorted_email = sorted(list(email))
                result = [name]
                result.extend(sorted_email)
                results.append(result)

        return results
