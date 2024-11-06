---
brief: Generate a unique email address in Flashduty, and synchronize the occurrence and recovery of alerts to Flashduty via email
---

# Email Integration

Generate a unique email address in Flashduty, and synchronize the occurrence and recovery of alerts to Flashduty via email.

## Operation Steps

### Create Email Integration

You can obtain an email address in two ways, choose either one.

#### Use Proprietary Integration

When you do not need to route alert events to different collaboration spaces, this method is preferred as it is simpler.

|+| Expand

    1. Enter the Flashduty console, select **Collaboration Space**, and enter the details page of a specific space
    2. Select the **Integrated Data** tab, click **Add an Integration**, and enter the Add Integration page
    3. Select **Email** Integration, click **Save**, and generate the card.
    4. Click on the generated card to view the **email address**, copy it for later use, and complete.

#### Use Shared Integration

When you need to route alerts to different collaboration spaces based on the payload information of the alert event, this method is preferred.

|+| Expand

    1. Enter the Flashduty console, select **Integration Center => Alert Events**, and enter the integration selection page.
    2. Select **Email** Integration:
    - **Integration Name**: Define a name for the current integration.
    - **Email Address**: Set a memorable prefix for the email address, which must be unique under your account.
    - **Push Mode**: Choose the conditions under which the email triggers or resolves alerts.
    3. Copy the **email address** on the current page for later use.
    4. Click **Create Route** to configure routing rules for the integration. You can match different alerts to different collaboration spaces based on conditions, or you can set a default collaboration space as a fallback and adjust it as needed.
    5. Complete.

### Customize Email Integration

#### Email Address

By default, the system will generate a unique email address for you. You can modify it, but note that **the email prefix can only consist of letters and numbers** and must be unique within your account.

#### Push Mode

By default, the system always creates a new alert for each email, but you can switch the mode to:

1. **Trigger or update based on the email subject**: In this mode, whenever a new email is received, the system will search for unclosed alerts based on the email subject. If an alert is found, it will be updated; otherwise, the system will trigger a new alert.
2. **Trigger or close based on rules**: In this mode, whenever a new email is received, the system will match the email according to your rules, and the matched emails will trigger new alerts or close existing alerts according to the rules.

- You need to fill in at least one **trigger** rule
- You must set the regular expression extraction rule for the Alert Key. The system uses this field to find historical alerts for updating or closing; **if the regular expression extraction fails, the system will use the email subject to generate the Alert Key** to ensure that alerts are not lost due to misconfiguration;
- You can choose whether to discard emails when none of the rules match.

Configuration Example:

- Receive all emails. When the email content contains the word **RESOLVED**, close the alert; otherwise, trigger a new alert
- The Alert Key is extracted from the email subject, with the rule being **/(.\*)/**.

<img src="https://fcdoc.github.io/img/en/flashduty/mixin/alert_integration/email/1.avif" alt="drawing" width="800"/>

### Things to Note

1. If the email message body is larger than 5MB, the system will directly refuse to receive it.
2. If the text content of the email exceeds 32KB, the system will truncate it and add a label prompt in the incident details:

```
body_cut = true
```

3. If the email contains an attachment, the system will directly discard the attachment and add a label prompt in the incident details:

```
attachment_stripped = true
```

4. In the new alert triggered by email, **the title is the email subject, and the description is the email content**.

5. If you change your account domain name, this email address will also change. Be sure to update your push address.

## Severity Mapping Relationship

The current alert levels pushed to Flashduty via email integration are all set to Warning.